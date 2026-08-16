//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1976/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1976(t543: f64, t74700: f64, t116: f64, t21813: f64, t5966: f64, t890: f64, t5962: f64, t1544: f64, t4537: f64, t5876: f64, t670: f64, t1448: f64, t6836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75305 = t74700 * t543;
    let t75439 = t21813 * t116;
    let t77408 = t5966 * t890;
    let t77425 = t5962 * t890;
    let t77441 = t1544 * t4537;
    let t85360 = t5876 * t670;
    let t86753 = t6836 * t1448;
    (t75305, t75439, t77408, t77425, t77441, t85360, t86753)
}
