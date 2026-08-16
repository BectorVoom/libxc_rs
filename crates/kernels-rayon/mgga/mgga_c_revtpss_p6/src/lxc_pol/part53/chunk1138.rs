//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1138/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1138(t2014: f64, t33651: f64, t7312: f64, t119578: f64, t27154: f64, t28167: f64, t37956: f64, t5627: f64, t33602: f64, t7003: f64, t25805: f64, t7735: f64) -> (f64, f64, f64, f64, f64) {
    let t125531 = 2.0_f64 * t2014 * t7312 * t33651;
    let t125532 = t119578 * t27154;
    let t125536 = 6.0_f64 * t28167 * t37956 * t5627;
    let t125537 = t33602 * t7003;
    let t125539 = t25805 * t7735;
    (t125531, t125532, t125536, t125537, t125539)
}
