//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1437/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1437(t17240: f64, t5052: f64, t1222: f64, t3636: f64, t5391: f64, t5381: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64, t480: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17241 = t17240 * t5052;
    let t17243 = t1222 * t17241 / 216.0_f64;
    let t17258 = 0.10162730220579493208e-2_f64 * t5391 * t3636;
    let t17260 = 0.19055119163586549765e-3_f64 * t5381 * t3636;
    let t17283 = t3666 * t1803;
    let t17288 = t5215 * t1208;
    let t17289 = t17288 * t225;
    let t17290 = t17289 * t480;
    (t17243, t17258, t17260, t17283, t17288, t17289, t17290)
}
