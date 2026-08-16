//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1189/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1189(t221: f64, t3829: f64, t9921: f64, t3978: f64, t3970: f64, t3989: f64, t4056: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t240: f64, t4000: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9923 = t9921 * t221 * t3829;
    let t9924 = t3978 * t9923;
    let t9926 = t3989 * t3970;
    let t9929 = t550 * t4056;
    let t9930 = t9929 * t543;
    let t9931 = t3992 * t9930;
    let t9932 = t2661 * t9931;
    let t9934 = t4000 * t240;
    (t9923, t9924, t9926, t9930, t9932, t9934)
}
