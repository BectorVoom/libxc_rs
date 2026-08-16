//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk901;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk902;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk903;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta185(t1216: f64, t1653: f64, t3578: f64, t1222: f64, t1731: f64, t1744: f64, t1202: f64, t1743: f64, t225: f64, t4940: f64, t68: f64, t484: f64, t1177: f64, t4729: f64, t1229: f64, t3247: f64, t3961: f64, t4582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4953, t4954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk901(t1216, t1653, t3578);
        let (t4957, t4959, t4961, t4964) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk902(t1222, t1731, t1744, t1202, t1743, t225, t4940);
        let t4965 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk903(t4964, t68);
        let (t4966, t4969, t4972, t4973, t4974) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk904(t484, t4965, t1177, t4729, t1229, t3247, t3961, t4582);
    (t4953, t4954, t4957, t4959, t4961, t4964, t4965, t4966, t4969, t4972, t4973, t4974)
}
