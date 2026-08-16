//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3934/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3934(t61014: f64, t75451: f64, t75676: f64, t75714: f64, t1455: f64, t6951: f64, t1464: f64, t6936: f64, t116: f64, t13514: f64, t1459: f64, t1461: f64, t18204: f64, t18211: f64, t18214: f64, t1916: f64, t21881: f64, t22544: f64, t22564: f64, t22565: f64, t2371: f64, t4158: f64, t572: f64, t5795: f64, t5801: f64, t5802: f64, t5805: f64, t670: f64, t6945: f64, t6948: f64) -> (f64, f64, f64, f64) {
    let t75716 = t61014 + t75451 + t75676 + t75714;
    let t75720 = t1455 * t6951;
    let t75727 = t6936 * t1464;
    let t75760 = 12.0_f64 * t116 * t21881 * t572 * t670 + 12.0_f64 * t13514 * t572 * t5801 + 6.0_f64 * t22564 * t2371 * t572 + 12.0_f64 * t1459 * t22565 + 6.0_f64 * t1461 * t22544 + 12.0_f64 * t18204 * t1916 + 12.0_f64 * t18211 * t1916 + 6.0_f64 * t18214 * t1916 + 6.0_f64 * t4158 * t6945 + 3.0_f64 * t4158 * t6948 + 24.0_f64 * t5795 * t5802 + 12.0_f64 * t5795 * t5805;
    (t75716, t75720, t75727, t75760)
}
