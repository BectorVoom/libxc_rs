//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 526/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk526(t103: f64, t1557: f64, t3188: f64, t3193: f64, t942: f64, t379: f64, t1902: f64, t432: f64, t920: f64, t1903: f64, t447: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3194 = t103 * t1557;
    let t3195 = t3194 * t3188;
    let t3196 = t3193 * t3195;
    let t3199 = t103 * t942;
    let t3200 = t3199 * t379;
    let t3201 = t1902 * t3200;
    let t3204 = t920 * t432;
    let t3205 = t1903 * t3204;
    let t3206 = t1902 * t3205;
    let t3210 = t447 * t986 * t379;
    (t3194, t3195, t3196, t3199, t3200, t3201, t3205, t3206, t3210)
}
