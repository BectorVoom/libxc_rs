//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 888/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk888(t13383: f64, t9744: f64, t446: f64, t1882: f64, t3714: f64, t13390: f64, t2354: f64, t13292: f64, t3281: f64, t13296: f64, t724: f64, t13301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13777 = t9744 * t13383;
    let t13778 = t446 * t13777;
    let t13780 = t1882 * t3714;
    let t13781 = 2.0_f64 / 27.0_f64 * t13780;
    let t13782 = t2354 * t13390;
    let t13783 = t446 * t13782;
    let t13785 = t2354 * t13292;
    let t13786 = t3281 * t13785;
    let t13788 = t724 * t13296;
    let t13789 = t446 * t13788;
    let t13791 = t724 * t13301;
    (t13778, t13780, t13781, t13783, t13786, t13789, t13791)
}
