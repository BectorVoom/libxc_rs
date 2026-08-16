//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1233/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1233(t122647: f64, t28067: f64, t8713: f64, t9593: f64, t28196: f64, t28198: f64, t28187: f64, t8698: f64, t32662: f64, t7898: f64, t28167: f64, t38099: f64, t5627: f64) -> (f64, f64, f64, f64, f64) {
    let t128266 = 3.0_f64 * t122647 * t28067;
    let t128267 = t8713 * t9593;
    let t128270 = 2.0_f64 * t28196 * t128267 * t28198;
    let t128273 = t8698 * t28187;
    let t128274 = t7898 * t32662;
    let t128277 = 6.0_f64 * t28167 * t38099 * t5627;
    (t128266, t128270, t128273, t128274, t128277)
}
