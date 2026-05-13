# Spike Conventions

Patterns and stack choices established across spike sessions. New spikes follow these unless the question requires otherwise.

## Build Optimization

- **Use sccache** for development - install with `cargo install sccache`
- Configure with `RUSTC_WRAPPER=sccache` and `SCCACHE_DIR=/path/to/cache`
- First build is slower but subsequent builds are 92% faster
- Works by caching CubeCL proc-macro expansion artifacts

## Build Notes

- CARGO_BUILD_JOBS=8 showed no improvement (parallelization overhead > benefit)
- codegen-units changes not tested (sccache solves the problem)
- 170 kernel crates use CubeCL #[cube] macros - main build bottleneck