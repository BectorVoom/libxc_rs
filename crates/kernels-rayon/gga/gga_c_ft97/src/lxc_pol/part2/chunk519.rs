//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 519/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk519(t3108: f64, t348: f64, t492: f64, t920: f64, t1910: f64, t1909: f64, t1766: f64, t965: f64, t473: f64, t91: f64, t1775: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3109 = t348 * t3108;
    let t3113 = t920 * t492;
    let t3114 = t1910 * t3113;
    let t3115 = t1909 * t3114;
    let t3119 = t1766 * t965;
    let t3121 = t91 * t3119 * t473;
    let t3125 = t1775 * t959;
    (t3109, t3114, t3115, t3119, t3121, t3125)
}
