//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1282;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1283;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta277<F: Float>(t25: F, t265: F, t394: F, t7642: F, t1409: F, t2116: F, t40: F, t7552: F, t1419: F, t337: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1653: F, t7286: F, t7285: F, t1716: F, t2123: F, t1751: F, t225: F, t497: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7992, t7997, t7998, t7999) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1282::<F>(t25, t265, t394, t7642, t1409, t2116, t40, t7552, t1419, t337, t1887, dens_threshold, rho0, zeta_threshold);
        let t8002 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1283::<F>(t1653, t7286);
        let (t8003, t8006, t8009, t8010) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1284::<F>(t7285, t8002, t1716, t2123, t1751, t225, t497);
    (t7992, t7997, t7998, t7999, t8002, t8003, t8006, t8009, t8010)
}
