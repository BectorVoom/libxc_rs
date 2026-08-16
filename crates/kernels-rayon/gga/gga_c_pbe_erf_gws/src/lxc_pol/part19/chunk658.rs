//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 658/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk658(t143: f64, t3644: f64, t2864: f64, t128: f64, t102: f64, t120: f64, t3637: f64, t506: f64, t10: f64, t1563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3645 = t143 * t3644;
    let t3648 = 0.97434166666666666666e0_f64 * t2864;
    let t3649 = t128 * t3644;
    let t3651 = 0.584605e1_f64 * t102 * t3649;
    let t3652 = t120 * t3637;
    let t3654 = 0.2923025e1_f64 * t102 * t3652;
    let t3656 = t506 * t3644;
    let t3657 = t10 * t3656;
    let t3660 = t128 * t3637;
    let t3661 = t10 * t3660;
    let t3665 = t1563 * t3644;
    let t3668 = t506 * t3637;
    (t3645, t3648, t3649, t3651, t3652, t3654, t3656, t3657, t3660, t3661, t3665, t3668)
}
