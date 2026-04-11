//! Safe wrapper functions for LDA_X CubeCL kernel launches.
//!
//! This module encapsulates all unsafe CubeCL operations for LDA_X kernels,
//! satisfying BUILD-04: all unsafe kernel launch code is confined to
//! `src/kernel/lda/`.
//!
//! Each wrapper function takes CubeCL handles and sizes, constructs ArrayArg
//! internally, and calls launch_unchecked -- all within a single unsafe block.
//! Callers (e.g., src/eval/dispatch.rs) never touch unsafe code.
//!
//! All wrappers return `Result` to propagate kernel launch failures instead
//! of panicking via `unwrap()`.

use cubecl::cpu::CpuRuntime;
use cubecl::client::ComputeClient;
use cubecl::prelude::*;

use super::lda_x;

/// A CubeCL buffer handle with its element count, used to pass array parameters
/// to safe wrapper functions without requiring unsafe ArrayArg construction
/// at the call site.
pub struct BufArg<'a> {
    pub handle: &'a cubecl::server::Handle,
    pub len: usize,
}

impl<'a> BufArg<'a> {
    pub fn new(handle: &'a cubecl::server::Handle, len: usize) -> Self {
        Self { handle, len }
    }
}

// ============================================================================
// UNPOLARIZED WRAPPERS
// ============================================================================

/// Safe wrapper for `lda_x_exc_unpol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_exc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_vxc_unpol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_vxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_vxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_fxc_unpol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_fxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_fxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_kxc_unpol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_kxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_kxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_lxc_unpol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_lxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    v4rho4: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ArrayArg::from_raw_parts::<f64>(v4rho4.handle, v4rho4.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

// ============================================================================
// POLARIZED WRAPPERS
// ============================================================================

/// Safe wrapper for `lda_x_exc_pol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_exc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_vxc_pol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_vxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_vxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_fxc_pol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_fxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_fxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_kxc_pol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_kxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_kxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

/// Safe wrapper for `lda_x_lxc_pol` kernel launch.
///
/// # Errors
/// Returns an error if the CubeCL kernel launch fails.
pub fn launch_lda_x_lxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    v4rho4: &BufArg<'_>,
    alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_x::lda_x_lxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ArrayArg::from_raw_parts::<f64>(v4rho4.handle, v4rho4.len, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::launch::{
        calculate_launch_config, cpu_client, create_input_buffer,
        create_zero_output_buffer, read_output_buffer,
    };

    /// Verify that the safe wrapper for exc_unpol produces correct results.
    #[test]
    fn test_launch_lda_x_exc_unpol_produces_negative_energy() {
        let client = cpu_client();
        let np = 4;
        let rho_data = [0.1, 0.2, 0.5, 1.0];

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        launch_lda_x_exc_unpol(
            &client, cube_count, cube_dim,
            &BufArg::new(&rho_handle, np),
            &BufArg::new(&zk_handle, np),
            1.0, 1e-15, 1e-10,
        ).unwrap();

        let zk = read_output_buffer(&client, zk_handle, np);
        assert_eq!(zk.len(), np);
        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    /// Verify vxc wrapper produces both zk and vrho output.
    #[test]
    fn test_launch_lda_x_vxc_unpol_produces_both_outputs() {
        let client = cpu_client();
        let np = 2;
        let rho_data = [0.1, 0.5];

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let vrho_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        launch_lda_x_vxc_unpol(
            &client, cube_count, cube_dim,
            &BufArg::new(&rho_handle, np),
            &BufArg::new(&zk_handle, np),
            &BufArg::new(&vrho_handle, np),
            1.0, 1e-15, 1e-10,
        ).unwrap();

        let zk = read_output_buffer(&client, zk_handle, np);
        let vrho = read_output_buffer(&client, vrho_handle, np);

        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
        for (i, &val) in vrho.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero");
        }
    }

    /// Verify polarized exc wrapper works correctly.
    #[test]
    fn test_launch_lda_x_exc_pol_produces_negative_energy() {
        let client = cpu_client();
        let np = 2;
        let rho_data = [0.1, 0.05, 0.2, 0.1];

        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        launch_lda_x_exc_pol(
            &client, cube_count, cube_dim,
            &BufArg::new(&rho_handle, np * 2),
            &BufArg::new(&zk_handle, np),
            1.0, 1e-15, 1e-10,
        ).unwrap();

        let zk = read_output_buffer(&client, zk_handle, np);
        for (i, &val) in zk.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }
}
