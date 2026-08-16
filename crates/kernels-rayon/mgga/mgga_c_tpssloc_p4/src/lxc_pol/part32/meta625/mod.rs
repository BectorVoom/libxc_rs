//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2033;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta625(t86843: f64, t4119: f64, t857: f64, t23168: f64, t25342: f64, t25345: f64, t82038: f64, t1519: f64, t213: f64, t225: f64, t25229: f64, t794: f64, t23164: f64, t6555: f64, t7480: f64, t81632: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t25059: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86844, t86849, t86869, t86870, t86873, t86887, t86893) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2033(t86843, t4119, t857, t23168, t25342, t25345, t82038, t1519, t213, t225, t25229, t794);
        let (t86896, t86903, t86911, t86916, t86928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2034(t23164, t6555, t86893, t7480, t81632, t23030, t25035, t23228, t7479, t81573, t25059, t6562, t794);
    (t86844, t86849, t86869, t86870, t86873, t86887, t86893, t86896, t86903, t86911, t86916, t86928)
}
