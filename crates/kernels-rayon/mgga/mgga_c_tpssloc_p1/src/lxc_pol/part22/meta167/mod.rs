//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1010;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1011;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1012;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta167(t1222: f64, t1731: f64, t1744: f64, t1202: f64, t1743: f64, t225: f64, t4940: f64, t68: f64, t484: f64, t1177: f64, t4729: f64, t1229: f64, t3247: f64, t3961: f64, t4582: f64, t1734: f64, t486: f64, t1215: f64, t3508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4957, t4959, t4961, t4964, t4965) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1010(t1222, t1731, t1744, t1202, t1743, t225, t4940, t68);
        let (t4966, t4969, t4972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1011(t484, t4965, t1177, t4729, t1229, t3247);
        let (t4973, t4974) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1012(t3961, t4972, t4582);
        let (t4977, t4978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1013(t1734, t486, t1215, t3508);
    (t4957, t4959, t4961, t4964, t4965, t4966, t4969, t4972, t4973, t4974, t4977, t4978)
}
