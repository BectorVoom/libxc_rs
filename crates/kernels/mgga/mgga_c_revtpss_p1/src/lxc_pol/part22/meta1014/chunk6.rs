//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3497/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3497<F: Float>(t11672: F, t19785: F, t1045: F, t4772: F, t1042: F, t1063: F, t11250: F, t11632: F, t11859: F, t11927: F, t19634: F, t19636: F, t19782: F, t19836: F, t20089: F, t3117: F, t3151: F, t42621: F, t42643: F, t43105: F, t4801: F, t4905: F, t53633: F, t53641: F, t53643: F, t54950: F, t60838: F, t6271: F) -> F {
    let t65892 = t11672 * t19785;
    let t65894 = t1045 * t4772;
    let t65929 = F::cast_from(0.3811023832717309953e-3_f64) * t53633 + F::cast_from(0.40650920882317972832e-2_f64) * t53641 - F::cast_from(0.33875767401931644026e-2_f64) * t53643 - F::cast_from(0.20325460441158986416e-2_f64) * t65892 + F::cast_from(0.17149607247227894789e-2_f64) * t11927 * t3117 * t4905 * t65894 - F::cast_from(0.17149607247227894789e-2_f64) * t42643 * t19636 - F::cast_from(0.17149607247227894789e-2_f64) * t11859 * t3117 * t19836 * t19634 - F::cast_from(0.17149607247227894789e-2_f64) * t11859 * t3117 * t20089 * t19634 - F::cast_from(0.85748036236139473944e-3_f64) * t11859 * t3117 * t6271 * t54950 - F::cast_from(0.25724410870841842183e-2_f64) * t42621 * t3117 * t6271 * t11632 * t3151 + F::cast_from(0.25724410870841842183e-2_f64) * t43105 * t3117 * t6271 * t11250 * t3151 - F::cast_from(0.2540682555144873302e-2_f64) * t11672 * t19782 - F::cast_from(0.57165357490759649296e-3_f64) * t1063 * t1042 * t4801 * t60838;
    t65929
}
