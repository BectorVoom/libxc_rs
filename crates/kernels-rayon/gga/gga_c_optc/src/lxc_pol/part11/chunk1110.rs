//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1110/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1110(t115: f64, t2770: f64, t5274: f64, t3209: f64, t1724: f64, t12597: f64, t4501: f64, t5336: f64, t7878: f64, t1179: f64, t5297: f64, t1162: f64, t5318: f64, t7274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45438 = t5274 * t2770 * t115;
    let t45439 = t3209 * t45438;
    let t45442 = t1724 * t45438;
    let t45584 = t4501 * t12597;
    let t45693 = t7878 * t5336;
    let t45694 = t1179 * t45693;
    let t45718 = t7878 * t5297;
    let t45719 = t1179 * t45718;
    let t45731 = t1162 * t7274 * t5318;
    (t45439, t45442, t45584, t45693, t45694, t45718, t45719, t45731)
}
