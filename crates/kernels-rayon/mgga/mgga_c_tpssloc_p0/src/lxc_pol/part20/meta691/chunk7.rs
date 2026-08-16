//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2630/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2630(t3566: f64, t5023: f64, t15734: f64, t3490: f64, t11789: f64, t1227: f64, t248: f64, t4733: f64, t11814: f64, t1232: f64, t15498: f64, t3527: f64, t3531: f64, t45264: f64, t45266: f64, t45271: f64, t45283: f64, t45296: f64, t5014: f64) -> f64 {
    let t53507 = t3566 * t5023;
    let t53515 = t3490 * t15734;
    let t53516 = t53515 / 6912.0_f64;
    let t53519 = t1227 * t248 * t11789 * t4733;
    let t53520 = t53519 / 6912.0_f64;
    let t53524 = -t45264 / 2304.0_f64 - t45266 / 2304.0_f64 - 5.0_f64 / 7776.0_f64 * t45271 + t53507 * t1232 / 288.0_f64 + t15498 * t3527 / 288.0_f64 + t15498 * t3531 / 144.0_f64 - t45283 / 768.0_f64 + t53516 + t53520 - t45296 / 5184.0_f64 + t11814 * t5014 / 1024.0_f64;
    t53524
}
