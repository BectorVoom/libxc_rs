//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3718/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3718(t1222: f64, t140: f64, t21209: f64, t21213: f64, t3685: f64, t12865: f64, t5436: f64, t1012: f64, t1225: f64, t12866: f64, t17634: f64, t17661: f64, t17693: f64, t17696: f64, t20771: f64, t20937: f64, t56981: f64, t57191: f64, t57209: f64, t57212: f64, t57214: f64, t57222: f64, t57227: f64, t57378: f64, t60754: f64) -> (f64, f64) {
    let t70491 = t1222 * t140 * t21209;
    let t70493 = t21213 * t3685;
    let t70496 = t5436 * t12865;
    let t70508 = -t1222 * t1012 * t1225 * t60754 / 288.0_f64 - 0.17149607247227894789e-2_f64 * t57191 + t57209 / 162.0_f64 + t57212 / 324.0_f64 - 2.0_f64 / 243.0_f64 * t57214 - t70491 / 432.0_f64 - 11.0_f64 / 486.0_f64 * t70493 + 0.19055119163586549765e-3_f64 * t57222 + 0.95275595817932748826e-3_f64 * t70496 * t17696 + 0.57165357490759649296e-3_f64 * t12866 * t17661 * t17634 - 0.76220476654346199061e-3_f64 * t57227 + 0.57165357490759649296e-3_f64 * t57378 * t20771 - 0.11433071498151929859e-2_f64 * t17693 * t56981 * t20937;
    (t70496, t70508)
}
