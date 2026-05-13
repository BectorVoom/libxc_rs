# Build Optimization

## Requirements

- Build time must be reduced significantly (target: >50% improvement on repeated builds)
- Must work with existing CubeCL proc-macro architecture
- Must not break existing tests or functionality

## How to Build It

### Setup sccache for Development

1. Install sccache:
   ```bash
   cargo install sccache
   ```

2. Configure in shell environment:
   ```bash
   export RUSTC_WRAPPER=sccache
   export SCCACHE_DIR=/tmp/sccache  # or persistent location
   ```

3. Build as normal - first build will be slightly slower, subsequent builds are cached.

### Verification

Test the setup by building the same crates twice:
```bash
# First build - cold cache
time cargo build -p libxc-kernel-gga-1a -p libxc-kernel-gga-1b -p libxc-kernel-gga-1c

# Second build - warm cache (should be ~92% faster)
time cargo build -p libxc-kernel-gga-1a -p libxc-kernel-gga-1b -p libxc-kernel-gga-1c
```

## What to Avoid

- **CARGO_BUILD_JOBS parallelization** - Testing showed no improvement, parallelization overhead exceeds benefit
- **codegen-units tuning** - Not necessary when sccache is used
- **Changing CubeCL proc-macro settings** - Could break kernel compilation

## Constraints

- sccache works by caching the output of the Rust compiler
- First build has ~30s overhead due to sccache coordination
- Cache is local to machine by default (can be configured for distributed caching)
- Works with any Cargo project using Rust compiler

## Origin

Synthesized from spikes: 001
Source files available in: sources/001-kernel-build-time/