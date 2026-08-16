//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3497/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3497(t11672: f64, t19785: f64, t1045: f64, t4772: f64, t1042: f64, t1063: f64, t11250: f64, t11632: f64, t11859: f64, t11927: f64, t19634: f64, t19636: f64, t19782: f64, t19836: f64, t20089: f64, t3117: f64, t3151: f64, t42621: f64, t42643: f64, t43105: f64, t4801: f64, t4905: f64, t53633: f64, t53641: f64, t53643: f64, t54950: f64, t60838: f64, t6271: f64) -> f64 {
    let t65892 = t11672 * t19785;
    let t65894 = t1045 * t4772;
    let t65929 = 0.3811023832717309953e-3_f64 * t53633 + 0.40650920882317972832e-2_f64 * t53641 - 0.33875767401931644026e-2_f64 * t53643 - 0.20325460441158986416e-2_f64 * t65892 + 0.17149607247227894789e-2_f64 * t11927 * t3117 * t4905 * t65894 - 0.17149607247227894789e-2_f64 * t42643 * t19636 - 0.17149607247227894789e-2_f64 * t11859 * t3117 * t19836 * t19634 - 0.17149607247227894789e-2_f64 * t11859 * t3117 * t20089 * t19634 - 0.85748036236139473944e-3_f64 * t11859 * t3117 * t6271 * t54950 - 0.25724410870841842183e-2_f64 * t42621 * t3117 * t6271 * t11632 * t3151 + 0.25724410870841842183e-2_f64 * t43105 * t3117 * t6271 * t11250 * t3151 - 0.2540682555144873302e-2_f64 * t11672 * t19782 - 0.57165357490759649296e-3_f64 * t1063 * t1042 * t4801 * t60838;
    t65929
}
