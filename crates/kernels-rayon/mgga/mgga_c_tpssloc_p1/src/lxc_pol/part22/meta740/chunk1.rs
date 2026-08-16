//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2438/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438(t21299: f64, t2844: f64, t2842: f64, t912: f64, t10702: f64, t17527: f64, t4395: f64, t21252: f64, t42100: f64, t42102: f64, t10811: f64, t14271: f64, t14276: f64, t17492: f64, t17544: f64, t17548: f64, t17551: f64, t21115: f64, t2930: f64, t4416: f64, t4438: f64, t4471: f64, t59920: f64, t60407: f64, t69335: f64, t69337: f64, t69340: f64, t69343: f64, t69346: f64, t931: f64) -> (f64, f64, f64, f64) {
    let t69347 = t21299 * t2844;
    let t69350 = 0.16081979498692535067e2_f64 * t2842 * t69347 * t912;
    let t69353 = 0.1551780387578202009e4_f64 * t10702 * t17527 * t4395;
    let t69357 = 0.24955700379505800916e5_f64 * t42100 * t21252 * t42102 * t912;
    let t69368 = 0.11579025239058625248e4_f64 * t10811 * t21115 * t931 + 0.51947577317044391277e2_f64 * t2930 * t17492 * t4471 - t69335 + t69337 + t69340 + t69343 + t69346 - t69350 - t69353 - t69357 - 6.0_f64 * t59920 * t4416 + 0.96491876992155210402e2_f64 * t60407 * t4438 - 6.0_f64 * t14276 * t17544 + 0.96491876992155210402e2_f64 * t14271 * t17548 + 0.1929837539843104208e3_f64 * t14271 * t17551;
    (t69350, t69353, t69357, t69368)
}
