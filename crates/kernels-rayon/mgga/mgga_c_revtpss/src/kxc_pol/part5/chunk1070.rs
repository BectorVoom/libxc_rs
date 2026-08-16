//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1070/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1070(t1469: f64, t2609: f64, t706: f64, t1568: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t4469: f64, t689: f64, t1579: f64, t2769: f64) -> (f64, f64, f64, f64) {
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    let t14474 = t2439 * t14473;
    let t14476 = t212 * t4469;
    let t14477 = t14476 * t780;
    let t14479 = 0.10975748638225852664e-1_f64 * t689 * t14477;
    let t14480 = t2769 * t1579;
    (t14441, t14474, t14479, t14480)
}
