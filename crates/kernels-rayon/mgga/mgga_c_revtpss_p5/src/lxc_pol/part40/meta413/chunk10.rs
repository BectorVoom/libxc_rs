//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1504/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1504(t2198: f64, t2371: f64, t670: f64, t8320: f64, t117: f64, t118019: f64, t13514: f64, t1459: f64, t1518: f64, t18204: f64, t18208: f64, t18211: f64, t18214: f64, t1916: f64, t2207: f64, t31231: f64, t31235: f64, t31238: f64, t31493: f64, t31494: f64, t31506: f64, t31509: f64, t4158: f64, t4162: f64, t4292: f64, t572: f64, t5805: f64, t8336: f64, t8342: f64, t8421: f64, t8427: f64) -> f64 {
    let t118157 = t2371 * t2198;
    let t118161 = t670 * t8320;
    let t118198 = 3.0_f64 * t117 * t118019 * t572 + 6.0_f64 * t118157 * t1518 * t572 + 12.0_f64 * t118161 * t1518 * t572 + 6.0_f64 * t13514 * t572 * t8342 + 12.0_f64 * t31493 * t4292 * t572 + 12.0_f64 * t1459 * t31494 + 12.0_f64 * t1459 * t31506 + 6.0_f64 * t1459 * t31509 + 6.0_f64 * t18204 * t2207 + 12.0_f64 * t18208 * t2207 + 6.0_f64 * t18211 * t2207 + 3.0_f64 * t18214 * t2207 + 6.0_f64 * t1916 * t31231 + 12.0_f64 * t1916 * t31235 + 6.0_f64 * t1916 * t31238 + 6.0_f64 * t4158 * t8427 + 6.0_f64 * t4162 * t8421 + 6.0_f64 * t5805 * t8336;
    t118198
}
