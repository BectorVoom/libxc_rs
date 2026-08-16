//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1586/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1586(t16750: f64, t482: f64, t371: f64, t372: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64, t480: f64, t3678: f64, t5327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17278 = t482 * t16750;
    let t17280 = t371 * t372 * t17278;
    let t17283 = t3666 * t1803;
    let t17288 = t5215 * t1208;
    let t17289 = t17288 * t225;
    let t17290 = t17289 * t480;
    let t17296 = 0.28582678745379824648e-3_f64 * t5327 * t3678;
    (t17280, t17283, t17288, t17289, t17290, t17296)
}
