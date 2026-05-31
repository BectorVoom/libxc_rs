//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3528/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3528<F: Float>(t127: F, t15700: F, t19979: F, t19981: F, t1011: F, t1045: F, t11774: F, t11927: F, t15591: F, t15696: F, t15907: F, t15917: F, t16043: F, t16081: F, t19611: F, t19620: F, t19626: F, t19836: F, t19861: F, t19982: F, t3115: F, t3117: F, t43066: F, t4915: F, t4919: F, t53923: F, t54651: F, t54656: F, t63253: F, t63364: F, t64848: F, t65192: F) -> F {
    let t66860 = t15700 * t127 * t19979 * t19981;
    let t66865 = -t1011 * t4919 * t63364 / F::cast_from(36.0_f64) - t1011 * t4915 * t63253 / F::cast_from(72.0_f64) + F::cast_from(0.51448821741683684367e-2_f64) * t16081 * t3117 * t15907 * t64848 - F::cast_from(0.28582678745379824648e-3_f64) * t15917 * t19626 + F::cast_from(0.3811023832717309953e-3_f64) * t54651 + F::cast_from(0.17149607247227894789e-2_f64) * t11927 * t3117 * t19836 * t19620 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t19611 * t16043 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t3117 * t65192 * t1045 + F::cast_from(0.20325460441158986416e-2_f64) * t54656 + F::cast_from(0.30488190661738479624e-2_f64) * t43066 * t19861 - F::cast_from(0.5081365110289746604e-2_f64) * t53923 * t19982 + F::cast_from(0.6351706387862183255e-3_f64) * t66860 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15696 * t15591;
    t66865
}
