//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 423/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk423(t3626: f64, t531: f64, t3630: f64, t3601: f64, t808: f64, t568: f64, t836: f64, t3614: f64, t2090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3655 = t531 * t3626;
    let t3658 = t531 * t3630;
    let t3661 = t808 * t3601;
    let t3662 = t568 * t3661;
    let t3666 = t836 * t3601;
    let t3667 = t568 * t3666;
    let t3670 = t808 * t3614;
    let t3671 = t568 * t3670;
    let t3676 = t2090 * t3601;
    let t3677 = t568 * t3676;
    let t3680 = t836 * t3614;
    let t3681 = t568 * t3680;
    (t3655, t3658, t3661, t3662, t3666, t3667, t3670, t3671, t3676, t3677, t3680, t3681)
}
