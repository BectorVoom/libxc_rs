//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2283/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2283<F: Float>(t1887: F, t80827: F, t26334: F, t26339: F, t81159: F, t22716: F, t7697: F, t16452: F, t26224: F, t26225: F, t80647: F, t80659: F, t80665: F, t80667: F, t80683: F, t90460: F, t90462: F, t90466: F, t90469: F, t90471: F, t90473: F, t90477: F, t90485: F, t90491: F, t90493: F, t90496: F) -> (F, F) {
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    let t90500 = t81159 * t26339;
    let t90501 = F::cast_from(0.76763589786250567036e-1_f64) * t90500;
    let t90503 = t22716 * t7697;
    let t90505 = t90460 + F::cast_from(0.3289868133696452873e-1_f64) * t90462 + F::cast_from(0.16449340668482264365e-1_f64) * t90466 + t90469 + t90471 - t90473 + F::cast_from(0.3289868133696452873e-1_f64) * t90477 - F::cast_from(12.0_f64) * t26224 * t26225 * t16452 + F::cast_from(0.82246703342411321824e-2_f64) * t80647 - F::cast_from(0.49348022005446793095e-1_f64) * t90485 + F::cast_from(0.82246703342411321824e-2_f64) * t80659 - F::cast_from(0.3289868133696452873e-1_f64) * t90491 - t90493 + F::cast_from(0.76763589786250567036e-1_f64) * t80665 + F::cast_from(0.38381794893125283518e-1_f64) * t80667 - t90496 - F::cast_from(0.2302907693587517011e0_f64) * t90498 - t90501 - F::cast_from(0.24674011002723396547e-1_f64) * t80683 + F::cast_from(0.63969658155208805863e-1_f64) * t90503;
    (t90497, t90505)
}
