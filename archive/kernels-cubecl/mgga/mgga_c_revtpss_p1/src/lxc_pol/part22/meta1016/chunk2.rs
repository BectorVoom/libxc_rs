//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3510/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3510<F: Float>(t11922: F, t20090: F, t3115: F, t19649: F, t372: F, t11774: F, t20039: F, t53405: F, t19837: F, t15691: F, t16068: F, t16082: F, t16095: F, t16096: F, t20075: F, t3092: F, t3096: F, t3117: F, t3151: F, t42765: F, t42804: F, t42872: F, t43069: F, t53676: F, t54078: F, t54081: F, t54085: F, t54316: F, t54509: F, t54811: F, t6092: F, t6266: F, t64891: F) -> F {
    let t66304 = t3115 * t11922 * t20090;
    let t66306 = t372 * t19649;
    let t66328 = t11774 * t53405 * t20039;
    let t66332 = t3115 * t11922 * t19837;
    let t66336 = F::cast_from(0.45732285992607719436e-2_f64) * t42765 * t20075 - F::cast_from(0.17149607247227894789e-2_f64) * t16095 * t3092 * t6092 * t16096 - F::cast_from(0.57165357490759649296e-3_f64) * t66304 + F::cast_from(0.57165357490759649296e-3_f64) * t43069 * t66306 * t3096 - F::cast_from(0.21437009059034868486e-3_f64) * t53676 * t3117 * t64891 * t16068 + F::cast_from(0.51448821741683684368e-2_f64) * t54509 * t3117 * t64891 * t42872 * t3151 - F::cast_from(0.77173232612525526552e-2_f64) * t54316 * t3117 * t64891 * t16082 + F::cast_from(0.28582678745379824648e-3_f64) * t54811 * t15691 * t42804 * t6266 - F::cast_from(0.3811023832717309953e-3_f64) * t66328 + F::cast_from(0.19055119163586549765e-3_f64) * t54078 - F::cast_from(0.57165357490759649296e-3_f64) * t66332 + F::cast_from(0.7622047665434619906e-3_f64) * t54081 - F::cast_from(0.6351706387862183255e-3_f64) * t54085;
    t66336
}
