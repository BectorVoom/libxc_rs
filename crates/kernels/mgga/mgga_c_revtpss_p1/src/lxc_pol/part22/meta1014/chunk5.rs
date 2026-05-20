//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3496/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3496<F: Float>(t15711: F, t4834: F, t4181: F, t999: F, t11860: F, t4866: F, t1063: F, t1066: F, t11859: F, t11991: F, t12004: F, t16089: F, t19968: F, t20046: F, t20094: F, t247: F, t3092: F, t3117: F, t3177: F, t3184: F, t3188: F, t42346: F, t4893: F, t53626: F, t53628: F, t6323: F, t6327: F, t63449: F) -> (F, F, F) {
    let t65859 = t4834 * t15711;
    let t65876 = t4181 * t999;
    let t65881 = t11860 * t4866;
    let t65888 = F::cast_from(0.80454947579587654563e-2_f64) * t12004 * t6327 - F::cast_from(0.6351706387862183255e-4_f64) * t65859 + F::cast_from(0.14291339372689912324e-3_f64) * t19968 * t3177 + F::cast_from(0.23818898954483187207e-3_f64) * t19968 * t3184 + F::cast_from(0.14291339372689912324e-3_f64) * t11991 * t6323 + F::cast_from(0.28582678745379824648e-3_f64) * t3188 * t20046 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t247 * t1066 * t63449 + F::cast_from(0.23818898954483187207e-3_f64) * t11991 * t6327 + F::cast_from(0.95275595817932748826e-4_f64) * t42346 - F::cast_from(0.22866142996303859718e-2_f64) * t16089 * t3092 * t20094 * t65876 - F::cast_from(0.17149607247227894789e-2_f64) * t11859 * t3117 * t4893 * t65881 + F::cast_from(0.19055119163586549765e-3_f64) * t53626 + F::cast_from(0.31758531939310916275e-3_f64) * t53628;
    (t65876, t65881, t65888)
}
