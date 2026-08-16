//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2769/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769(t22298: f64, t48100: f64, t9816: f64, t22129: f64, t2713: f64, t3964: f64, t22169: f64, t46691: f64, t22173: f64, t9744: f64, t6856: f64, t9779: f64) -> (f64, f64, f64, f64, f64) {
    let t74257 = t9816 * t48100 * t22298;
    let t74264 = t3964 * t2713 * t22129;
    let t74269 = t46691 * t22169;
    let t74271 = t9744 * t22173;
    let t74277 = t9779 * t6856;
    (t74257, t74264, t74269, t74271, t74277)
}
