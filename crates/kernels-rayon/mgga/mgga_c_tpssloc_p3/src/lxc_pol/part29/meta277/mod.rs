//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1282;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1283;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta277(t25: f64, t265: f64, t394: f64, t7642: f64, t1409: f64, t2116: f64, t40: f64, t7552: f64, t1419: f64, t337: f64, t1887: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1653: f64, t7286: f64, t7285: f64, t1716: f64, t2123: f64, t1751: f64, t225: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7992, t7997, t7998, t7999) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1282(t25, t265, t394, t7642, t1409, t2116, t40, t7552, t1419, t337, t1887, dens_threshold, rho0, zeta_threshold);
        let t8002 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1283(t1653, t7286);
        let (t8003, t8006, t8009, t8010) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1284(t7285, t8002, t1716, t2123, t1751, t225, t497);
    (t7992, t7997, t7998, t7999, t8002, t8003, t8006, t8009, t8010)
}
