//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3257/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3257(t49834: f64, t60183: f64, t60213: f64, t60558: f64, t1518: f64, t670: f64, t10259: f64, t116: f64, t117: f64, t13232: f64, t13240: f64, t13244: f64, t13247: f64, t13514: f64, t1459: f64, t1461: f64, t18190: f64, t18204: f64, t18207: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t1918: f64, t2327: f64, t2371: f64, t4158: f64, t4162: f64, t4165: f64, t4292: f64, t49830: f64, t572: f64, t573: f64, t5795: f64, t5801: f64, t5802: f64, t5805: f64, param_d: f64) -> (f64, f64) {
    let t60560 = t49834 + t60183 + t60213 + t60558;
    let t60595 = t670 * t1518;
    let t60599 = 18.0_f64 * t5795 * t4162 + 18.0_f64 * t1916 * t13244 + 3.0_f64 * t1916 * t13247 + 3.0_f64 * t13232 * t1918 + 9.0_f64 * t5795 * t4165 + param_d * t60560 * t573 + 9.0_f64 * t18190 * t1461 + 18.0_f64 * t572 * t116 * t13514 * t670 + 18.0_f64 * t572 * t18207 * t2371 + 18.0_f64 * t1459 * t18204 + 36.0_f64 * t1459 * t18208 + 18.0_f64 * t1459 * t18211 + 18.0_f64 * t4158 * t5802 + 3.0_f64 * t572 * t117 * t49830 + 9.0_f64 * t4158 * t5805 + 6.0_f64 * t1916 * t13240 + 18.0_f64 * t572 * t2327 * t4292 + 6.0_f64 * t572 * t5801 * t10259 + 9.0_f64 * t1459 * t18214 + 18.0_f64 * t572 * t60595 * t2371;
    (t60560, t60599)
}
