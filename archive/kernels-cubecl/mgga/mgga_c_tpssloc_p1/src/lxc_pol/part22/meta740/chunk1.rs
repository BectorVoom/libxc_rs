//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2438/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438<F: Float>(t21299: F, t2844: F, t2842: F, t912: F, t10702: F, t17527: F, t4395: F, t21252: F, t42100: F, t42102: F, t10811: F, t14271: F, t14276: F, t17492: F, t17544: F, t17548: F, t17551: F, t21115: F, t2930: F, t4416: F, t4438: F, t4471: F, t59920: F, t60407: F, t69335: F, t69337: F, t69340: F, t69343: F, t69346: F, t931: F) -> (F, F, F, F) {
    let t69347 = t21299 * t2844;
    let t69350 = F::cast_from(0.16081979498692535067e2_f64) * t2842 * t69347 * t912;
    let t69353 = F::cast_from(0.1551780387578202009e4_f64) * t10702 * t17527 * t4395;
    let t69357 = F::cast_from(0.24955700379505800916e5_f64) * t42100 * t21252 * t42102 * t912;
    let t69368 = F::cast_from(0.11579025239058625248e4_f64) * t10811 * t21115 * t931 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t17492 * t4471 - t69335 + t69337 + t69340 + t69343 + t69346 - t69350 - t69353 - t69357 - F::cast_from(6.0_f64) * t59920 * t4416 + F::cast_from(0.96491876992155210402e2_f64) * t60407 * t4438 - F::cast_from(6.0_f64) * t14276 * t17544 + F::cast_from(0.96491876992155210402e2_f64) * t14271 * t17548 + F::cast_from(0.1929837539843104208e3_f64) * t14271 * t17551;
    (t69350, t69353, t69357, t69368)
}
