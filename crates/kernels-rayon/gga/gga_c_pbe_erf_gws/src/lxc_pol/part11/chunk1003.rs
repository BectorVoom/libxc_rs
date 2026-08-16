//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1003/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1003(t19733: f64, t3912: f64, t833: f64, t3916: f64, t4423: f64, t19894: f64, t3717: f64, t945: f64, t3928: f64, t6854: f64, t1033: f64, t11025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39653 = t3912 * t19733 * t833;
    let t39661 = t3916 * t4423 * t833;
    let t39689 = t3912 * t19894;
    let t39749 = t945 * t3717;
    let t39758 = t3928 * t6854;
    let t39870 = t1033 * t11025;
    (t39653, t39661, t39689, t39749, t39758, t39870)
}
