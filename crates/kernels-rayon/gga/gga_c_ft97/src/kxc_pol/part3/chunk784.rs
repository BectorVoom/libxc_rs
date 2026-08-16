//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 784/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk784(t16169: f64, t3187: f64, t1909: f64, t4612: f64, t8506: f64, t3255: f64, t920: f64, t1910: f64, t18: f64, t979: f64, t432: f64, t452: f64, t4623: f64) -> (f64, f64, f64, f64, f64) {
    let t16170 = t3187 * t16169;
    let t16171 = t1909 * t16170;
    let t16174 = t8506 * t4612;
    let t16177 = t920 * t3255;
    let t16178 = t1910 * t16177;
    let t16179 = t1909 * t16178;
    let t16182 = t18 * t979;
    let t16183 = t1910 * t16182;
    let t16184 = t1909 * t16183;
    let t16188 = t452 * t4623 * t432;
    (t16171, t16174, t16179, t16184, t16188)
}
