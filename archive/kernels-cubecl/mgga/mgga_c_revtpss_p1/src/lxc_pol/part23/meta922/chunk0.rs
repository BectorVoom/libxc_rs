//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2979/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2979<F: Float>(t1651: F, t19380: F, t1043: F, t23598: F, t23916: F, t3091: F, t43131: F, t1045: F, t11703: F, t15830: F, t15850: F, t16067: F, t16089: F, t16095: F, t19450: F, t19716: F, t19968: F, t20046: F, t247: F, t3092: F, t3115: F, t3116: F, t3117: F, t4578: F, t4757: F, t4831: F, t4834: F, t4837: F, t4894: F, t4907: F, t53294: F, t53669: F, t6092: F, t6323: F, t6331: F, t65454: F, t65456: F, t65459: F, t65462: F, t65471: F, t67551: F, t78524: F, t78616: F, t78812: F) -> (F, F, F) {
    let t78826 = t1651 * t19380;
    let t78831 = t23598 * t1043;
    let t78855 = t3091 * t43131 * t23916;
    let t78857 = F::cast_from(0.64311027177104605458e-3_f64) * t16067 * t3117 * t19450 * t19716 + F::cast_from(0.30011812682648815881e-2_f64) * t53669 * t3117 * t78812 * t4894 + F::cast_from(0.42874018118069736972e-3_f64) * t19968 * t4831 + F::cast_from(0.42874018118069736972e-3_f64) * t15850 * t6323 - F::cast_from(0.57165357490759649295e-3_f64) * t65454 + F::cast_from(0.42874018118069736972e-3_f64) * t4834 * t20046 + F::cast_from(0.45732285992607719437e-2_f64) * t15830 * t6331 + F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t247 * t3116 * t78826 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t3117 * t78831 * t1045 - F::cast_from(0.64311027177104605458e-3_f64) * t67551 * t4907 + F::cast_from(0.14291339372689912324e-2_f64) * t16089 * t11703 * t6092 * t4757 - F::cast_from(0.25724410870841842183e-2_f64) * t16095 * t3092 * t6092 * t78524 + F::cast_from(0.17149607247227894789e-2_f64) * t16095 * t3092 * t4578 * t78616 - F::cast_from(0.45732285992607719437e-2_f64) * t65456 - F::cast_from(0.28582678745379824648e-2_f64) * t65459 + F::cast_from(0.95275595817932748826e-3_f64) * t65462 - t53294 - F::cast_from(0.57165357490759649295e-3_f64) * t65471 + F::cast_from(0.47637797908966374413e-3_f64) * t78855;
    (t78826, t78831, t78857)
}
