//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3496/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3496(t15711: f64, t4834: f64, t4181: f64, t999: f64, t11860: f64, t4866: f64, t1063: f64, t1066: f64, t11859: f64, t11991: f64, t12004: f64, t16089: f64, t19968: f64, t20046: f64, t20094: f64, t247: f64, t3092: f64, t3117: f64, t3177: f64, t3184: f64, t3188: f64, t42346: f64, t4893: f64, t53626: f64, t53628: f64, t6323: f64, t6327: f64, t63449: f64) -> (f64, f64, f64) {
    let t65859 = t4834 * t15711;
    let t65876 = t4181 * t999;
    let t65881 = t11860 * t4866;
    let t65888 = 0.80454947579587654563e-2_f64 * t12004 * t6327 - 0.6351706387862183255e-4_f64 * t65859 + 0.14291339372689912324e-3_f64 * t19968 * t3177 + 0.23818898954483187207e-3_f64 * t19968 * t3184 + 0.14291339372689912324e-3_f64 * t11991 * t6323 + 0.28582678745379824648e-3_f64 * t3188 * t20046 + 0.14291339372689912324e-3_f64 * t1063 * t247 * t1066 * t63449 + 0.23818898954483187207e-3_f64 * t11991 * t6327 + 0.95275595817932748826e-4_f64 * t42346 - 0.22866142996303859718e-2_f64 * t16089 * t3092 * t20094 * t65876 - 0.17149607247227894789e-2_f64 * t11859 * t3117 * t4893 * t65881 + 0.19055119163586549765e-3_f64 * t53626 + 0.31758531939310916275e-3_f64 * t53628;
    (t65876, t65881, t65888)
}
