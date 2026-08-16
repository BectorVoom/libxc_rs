//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3709/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3709(t1235: f64, t371: f64, t6645: f64, t676: f64, t21063: f64, t3678: f64, t17307: f64, t1803: f64, t17225: f64, t5381: f64, t1261: f64, t20791: f64, t3172: f64) -> (f64, f64, f64, f64, f64) {
    let t70263 = t1235 * t371 * t676 * t6645;
    let t70265 = t21063 * t3678;
    let t70267 = t17307 * t1803;
    let t70270 = t5381 * t17225;
    let t70273 = t1261 * t3172 * t20791;
    (t70263, t70265, t70267, t70270, t70273)
}
