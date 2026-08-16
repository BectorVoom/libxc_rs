//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3510/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510(t11922: f64, t20090: f64, t3115: f64, t19649: f64, t372: f64, t11774: f64, t20039: f64, t53405: f64, t19837: f64, t15691: f64, t16068: f64, t16082: f64, t16095: f64, t16096: f64, t20075: f64, t3092: f64, t3096: f64, t3117: f64, t3151: f64, t42765: f64, t42804: f64, t42872: f64, t43069: f64, t53676: f64, t54078: f64, t54081: f64, t54085: f64, t54316: f64, t54509: f64, t54811: f64, t6092: f64, t6266: f64, t64891: f64) -> f64 {
    let t66304 = t3115 * t11922 * t20090;
    let t66306 = t372 * t19649;
    let t66328 = t11774 * t53405 * t20039;
    let t66332 = t3115 * t11922 * t19837;
    let t66336 = 0.45732285992607719436e-2_f64 * t42765 * t20075 - 0.17149607247227894789e-2_f64 * t16095 * t3092 * t6092 * t16096 - 0.57165357490759649296e-3_f64 * t66304 + 0.57165357490759649296e-3_f64 * t43069 * t66306 * t3096 - 0.21437009059034868486e-3_f64 * t53676 * t3117 * t64891 * t16068 + 0.51448821741683684368e-2_f64 * t54509 * t3117 * t64891 * t42872 * t3151 - 0.77173232612525526552e-2_f64 * t54316 * t3117 * t64891 * t16082 + 0.28582678745379824648e-3_f64 * t54811 * t15691 * t42804 * t6266 - 0.3811023832717309953e-3_f64 * t66328 + 0.19055119163586549765e-3_f64 * t54078 - 0.57165357490759649296e-3_f64 * t66332 + 0.7622047665434619906e-3_f64 * t54081 - 0.6351706387862183255e-3_f64 * t54085;
    t66336
}
