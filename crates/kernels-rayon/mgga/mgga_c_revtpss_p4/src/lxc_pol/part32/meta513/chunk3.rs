//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1812/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1812(t5: f64, t30551: f64, t117: f64, t118: f64, t18245: f64, t1911: f64, t2014: f64, t2056: f64, t2093: f64, t2108: f64, t25082: f64, t29506: f64, t29508: f64, t30138: f64, t30209: f64, t30218: f64, t30315: f64, t30511: f64, t30513: f64, t4248: f64, t508: f64, t5887: f64, t651: f64, t6934: f64, t7359: f64, t7732: f64, t7898: f64, t7978: f64, t7984: f64, t8075: f64, t8079: f64, t8109: f64, t8111: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t30552 = piecewise3(t8, 0.0_f64, t30551);
    let t30553 = t30552 * t117;
    let t30555 = -t118 * t30511 - 2.0_f64 * t18245 * t2056 + 2.0_f64 * t1911 * t8075 - 2.0_f64 * t2014 * t30218 + t2014 * t30315 - 2.0_f64 * t2056 * t29508 - 4.0_f64 * t2056 * t30138 + t2093 * t6934 + t2108 * t29506 - 6.0_f64 * t25082 * t30513 - 4.0_f64 * t30209 * t651 - t30553 * t508 - 4.0_f64 * t4248 * t7978 - 4.0_f64 * t4248 * t7984 - 4.0_f64 * t5887 * t7359 - 4.0_f64 * t7732 * t7978 + 6.0_f64 * t7898 * t8079 + 2.0_f64 * t7898 * t8109 - 2.0_f64 * t7898 * t8111;
    (t30552, t30553, t30555)
}
