//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2450/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2450(t10231: f64, t13528: f64, t973: f64, t13532: f64, t13537: f64, t42972: f64, t135: f64, t14197: f64, t10863: f64, t14015: f64, t14018: f64, t14174: f64, t14180: f64, t14198: f64, t2960: f64, t2979: f64, t3048: f64, t4590: f64, t47684: f64, t47759: f64, t47763: f64, t977: f64) -> f64 {
    let t50110 = t973 * t10231 * t13528;
    let t50113 = t973 * t10231 * t13532;
    let t50116 = t973 * t42972 * t13537;
    let t50132 = t973 * t135 * t14197;
    let t50136 = 5.0_f64 / 144.0_f64 * t3048 * t14174 - 5.0_f64 / 432.0_f64 * t10863 * t4590 - 5.0_f64 / 432.0_f64 * t3048 * t14180 + t50110 / 108.0_f64 + t50113 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t50116 - t973 * t977 * t47759 / 48.0_f64 - t973 * t977 * t47763 / 48.0_f64 - t973 * t2979 * t47684 / 12.0_f64 - t2960 * t14015 / 27.0_f64 - 7.0_f64 / 81.0_f64 * t2960 * t14018 + t50132 / 288.0_f64 - t2960 * t14198 / 36.0_f64;
    t50136
}
