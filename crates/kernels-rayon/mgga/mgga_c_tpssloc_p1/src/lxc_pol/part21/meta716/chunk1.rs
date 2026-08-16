//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2557/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2557(t10263: f64, t4603: f64, t10891: f64, t13970: f64, t10231: f64, t13528: f64, t973: f64, t13532: f64, t13537: f64, t42972: f64, t135: f64, t14197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50098 = t10263 * t4603;
    let t50100 = t10891 * t13970;
    let t50110 = t973 * t10231 * t13528;
    let t50113 = t973 * t10231 * t13532;
    let t50116 = t973 * t42972 * t13537;
    let t50132 = t973 * t135 * t14197;
    (t50098, t50100, t50110, t50113, t50116, t50132)
}
