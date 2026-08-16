//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3528/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3528(t127: f64, t15700: f64, t19979: f64, t19981: f64, t1011: f64, t1045: f64, t11774: f64, t11927: f64, t15591: f64, t15696: f64, t15907: f64, t15917: f64, t16043: f64, t16081: f64, t19611: f64, t19620: f64, t19626: f64, t19836: f64, t19861: f64, t19982: f64, t3115: f64, t3117: f64, t43066: f64, t4915: f64, t4919: f64, t53923: f64, t54651: f64, t54656: f64, t63253: f64, t63364: f64, t64848: f64, t65192: f64) -> f64 {
    let t66860 = t15700 * t127 * t19979 * t19981;
    let t66865 = -t1011 * t4919 * t63364 / 36.0_f64 - t1011 * t4915 * t63253 / 72.0_f64 + 0.51448821741683684367e-2_f64 * t16081 * t3117 * t15907 * t64848 - 0.28582678745379824648e-3_f64 * t15917 * t19626 + 0.3811023832717309953e-3_f64 * t54651 + 0.17149607247227894789e-2_f64 * t11927 * t3117 * t19836 * t19620 - 0.21437009059034868486e-3_f64 * t3115 * t3117 * t19611 * t16043 - 0.42874018118069736972e-3_f64 * t3115 * t3117 * t65192 * t1045 + 0.20325460441158986416e-2_f64 * t54656 + 0.30488190661738479624e-2_f64 * t43066 * t19861 - 0.5081365110289746604e-2_f64 * t53923 * t19982 + 0.6351706387862183255e-3_f64 * t66860 - 0.28582678745379824648e-3_f64 * t11774 * t15696 * t15591;
    t66865
}
