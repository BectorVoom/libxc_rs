//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3599/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3599(t20340: f64, t698: f64, t20377: f64, t5079: f64, t3407: f64, t43911: f64, t56176: f64, t56183: f64, t56185: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64, t68363: f64, t68366: f64) -> (f64, f64, f64, f64, f64) {
    let t68368 = t698 * t20340;
    let t68370 = t698 * t20377;
    let t68372 = t5079 * t5079;
    let t68373 = t3407 * t68372;
    let t68379 = 0.33547222222222222222e0_f64 * t68342 + 0.40256666666666666666e1_f64 * t68347 - 0.12077e1_f64 * t68350 - 0.72462e1_f64 * t68353 - 0.40256666666666666666e0_f64 * t68357 + 0.72462e1_f64 * t68360 - 0.48307999999999999999e1_f64 * t68363 + 0.13418888888888888889e1_f64 * t68366 - 0.22076e0_f64 * t68368 - 0.49057777777777777778e-1_f64 * t68370 + 0.16504875e0_f64 * t68373 - 0.30661111111111111111e-1_f64 * t43911 - 0.35783703703703703705e0_f64 * t56176 + 0.10735111111111111112e1_f64 * t56183 - 0.80513333333333333336e0_f64 * t56185;
    (t68368, t68370, t68372, t68373, t68379)
}
