ARG UBUNTU_IMAGE=ubuntu:26.04
FROM ${UBUNTU_IMAGE}

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN set -eux; \
    \
    # 目的の GID に対応するグループ名を決める
    group_name="$(getent group "${USER_GID}" | cut -d: -f1 || true)"; \
    if [ -z "${group_name}" ]; then \
    groupadd --gid "${USER_GID}" "${USERNAME}"; \
    group_name="${USERNAME}"; \
    fi; \
    \
    # USERNAME が既に存在する場合は、そのユーザーを目的の UID/GID に寄せる
    if id -u "${USERNAME}" >/dev/null 2>&1; then \
    current_uid="$(id -u "${USERNAME}")"; \
    current_gid="$(id -g "${USERNAME}")"; \
    \
    if [ "${current_gid}" != "${USER_GID}" ]; then \
    usermod --gid "${USER_GID}" "${USERNAME}"; \
    fi; \
    \
    if [ "${current_uid}" != "${USER_UID}" ]; then \
    uid_owner="$(getent passwd "${USER_UID}" | cut -d: -f1 || true)"; \
    if [ -n "${uid_owner}" ] && [ "${uid_owner}" != "${USERNAME}" ]; then \
    echo "ERROR: UID ${USER_UID} is already used by user '${uid_owner}'" >&2; \
    exit 1; \
    fi; \
    usermod --uid "${USER_UID}" "${USERNAME}"; \
    fi; \
    else \
    # USERNAME が存在しない場合:
    # まず UID の既存所有者を探し、いればそのユーザーを USERNAME に寄せる
    uid_owner="$(getent passwd "${USER_UID}" | cut -d: -f1 || true)"; \
    if [ -n "${uid_owner}" ]; then \
    usermod --login "${USERNAME}" \
    --home "/home/${USERNAME}" \
    --move-home \
    --shell /bin/bash \
    --gid "${USER_GID}" \
    "${uid_owner}"; \
    else \
    useradd --uid "${USER_UID}" \
    --gid "${USER_GID}" \
    --create-home \
    --shell /bin/bash \
    "${USERNAME}"; \
    fi; \
    fi; \
    \
    mkdir -p "/home/${USERNAME}" /workspace /etc/sudoers.d; \
    chown -R "${USER_UID}:${USER_GID}" "/home/${USERNAME}" /workspace; \
    echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}"; \
    chmod 0440 "/etc/sudoers.d/${USERNAME}"


ENV TZ=Asia/Tokyo \
    LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    CARGO_HOME=/home/${USERNAME}/.cargo \
    RUSTUP_HOME=/home/${USERNAME}/.rustup \
    BUN_INSTALL=/home/${USERNAME}/.bun \
    UV_CACHE_DIR=/home/${USERNAME}/.cache/uv

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    bash-completion \
    build-essential \
    libssl-dev \
    ca-certificates \
    clang \
    curl \
    file \
    git \
    gpg \
    less \
    locales \
    lsb-release \
    mold \
    pkg-config \
    python3 \
    python3-pip \
    python3-venv \
    ripgrep \
    sudo \
    unzip \
    xz-utils \
    zip \
    nodejs \
    cmake \
    && ln -sf /usr/bin/python3 /usr/local/bin/python \
    && ln -sf /usr/bin/pip3 /usr/local/bin/pip \
    && locale-gen C.UTF-8 \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sccache --locked
USER ${USERNAME}
WORKDIR /home/${USERNAME}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable \
    && mkdir -p /home/${USERNAME}/.cargo \
    && cat > /home/${USERNAME}/.cargo/config.toml <<'CARGOEOF'
[build]
compiler = "sccache"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.aarch64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
CARGOEOF

RUN curl -LsSf https://astral.sh/uv/install.sh \
    | env UV_UNMANAGED_INSTALL="/home/${USERNAME}/.local/bin" sh

RUN curl -fsSL https://bun.sh/install | bash

ENV PATH=/home/${USERNAME}/.local/bin:/home/${USERNAME}/.cargo/bin:/home/${USERNAME}/.bun/bin:${PATH}

RUN bun add -g @openai/codex @anthropic-ai/claude-code \
    && rustup component add rustfmt clippy

WORKDIR /workspace

CMD ["/bin/bash", "-lc", "sleep infinity"]
