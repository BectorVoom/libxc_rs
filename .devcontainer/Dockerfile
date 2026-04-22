ARG UBUNTU_IMAGE=ubuntu:26.04
FROM ${UBUNTU_IMAGE}

ARG DEBIAN_FRONTEND=noninteractive
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# ---- Base packages ---------------------------------------------------------
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    bash-completion \
    build-essential \
    ca-certificates \
    clang \
    cmake \
    curl \
    file \
    git \
    gpg \
    less \
    libssl-dev \
    locales \
    lsb-release \
    mold \
    nodejs \
    pkg-config \
    python3 \
    python3-pip \
    python3-venv \
    ripgrep \
    sudo \
    unzip \
    xz-utils \
    zip; \
    ln -sf /usr/bin/python3 /usr/local/bin/python; \
    ln -sf /usr/bin/pip3 /usr/local/bin/pip; \
    locale-gen C.UTF-8; \
    rm -rf /var/lib/apt/lists/*

# ---- Create/update non-root user ------------------------------------------
RUN set -eux; \
    \
    # group
    if getent group "${USER_GID}" >/dev/null 2>&1; then \
    GROUP_NAME="$(getent group "${USER_GID}" | cut -d: -f1)"; \
    else \
    groupadd --gid "${USER_GID}" "${USERNAME}"; \
    GROUP_NAME="${USERNAME}"; \
    fi; \
    \
    # user
    if id -u "${USERNAME}" >/dev/null 2>&1; then \
    CURRENT_UID="$(id -u "${USERNAME}")"; \
    CURRENT_GID="$(id -g "${USERNAME}")"; \
    if [ "${CURRENT_GID}" != "${USER_GID}" ]; then \
    usermod --gid "${USER_GID}" "${USERNAME}"; \
    fi; \
    if [ "${CURRENT_UID}" != "${USER_UID}" ]; then \
    UID_OWNER="$(getent passwd "${USER_UID}" | cut -d: -f1 || true)"; \
    if [ -n "${UID_OWNER}" ] && [ "${UID_OWNER}" != "${USERNAME}" ]; then \
    echo "ERROR: UID ${USER_UID} is already used by user '${UID_OWNER}'" >&2; \
    exit 1; \
    fi; \
    usermod --uid "${USER_UID}" "${USERNAME}"; \
    fi; \
    else \
    UID_OWNER="$(getent passwd "${USER_UID}" | cut -d: -f1 || true)"; \
    if [ -n "${UID_OWNER}" ]; then \
    usermod --login "${USERNAME}" \
    --home "/home/${USERNAME}" \
    --move-home \
    --shell /bin/bash \
    --gid "${USER_GID}" \
    "${UID_OWNER}"; \
    else \
    useradd --uid "${USER_UID}" \
    --gid "${USER_GID}" \
    --create-home \
    --shell /bin/bash \
    "${USERNAME}"; \
    fi; \
    fi; \
    \
    install -d -m 0755 -o "${USER_UID}" -g "${USER_GID}" \
    /workspace \
    "/home/${USERNAME}/.cargo" \
    "/home/${USERNAME}/.cargo/registry" \
    "/home/${USERNAME}/.cargo/git" \
    "/home/${USERNAME}/.rustup" \
    "/home/${USERNAME}/.cache" \
    "/home/${USERNAME}/.cache/uv" \
    "/home/${USERNAME}/.cache/sccache" \
    "/home/${USERNAME}/.local/bin" \
    "/home/${USERNAME}/.bun" \
    "/home/${USERNAME}/.bun/install" \
    "/home/${USERNAME}/.bun/install/cache" \
    /etc/sudoers.d; \
    echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}"; \
    chmod 0440 "/etc/sudoers.d/${USERNAME}"

ENV TZ=Asia/Tokyo \
    LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    HOME=/home/${USERNAME} \
    CARGO_HOME=/home/${USERNAME}/.cargo \
    RUSTUP_HOME=/home/${USERNAME}/.rustup \
    BUN_INSTALL=/home/${USERNAME}/.bun \
    UV_CACHE_DIR=/home/${USERNAME}/.cache/uv \
    SCCACHE_DIR=/home/${USERNAME}/.cache/sccache

USER ${USERNAME}
WORKDIR /home/${USERNAME}

# PATH must include cargo before cargo install
ENV PATH=/home/${USERNAME}/.local/bin:/home/${USERNAME}/.cargo/bin:/home/${USERNAME}/.bun/bin:${PATH}

# ---- Rust ------------------------------------------------------------------
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable; \
    rustup component add rustfmt clippy; \
    cargo install sccache --locked

# ---- Cargo config ----------------------------------------------------------
RUN set -eux; \
    cat > "/home/${USERNAME}/.cargo/config.toml" <<'CARGOEOF'
[build]
rustc-wrapper = "sccache"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.aarch64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
CARGOEOF

# ---- uv --------------------------------------------------------------------
RUN set -eux; \
    curl -LsSf https://astral.sh/uv/install.sh \
    | env UV_UNMANAGED_INSTALL="/home/${USERNAME}/.local/bin" sh

# ---- bun -------------------------------------------------------------------
RUN set -eux; \
    curl -fsSL https://bun.sh/install | bash; \
    bun add -g @openai/codex @anthropic-ai/claude-code

WORKDIR /workspace

CMD ["/bin/bash", "-lc", "sleep infinity"]
