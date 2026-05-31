//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 967/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk967<F: Float>(t322: F, t1338: F, t3552: F, t1142: F, t6755: F, t11216: F, t1348: F, t6767: F, t11239: F, t11244: F, t1125: F, t11273: F, t11298: F, t1307: F, t2438: F, t330: F, t3517: F, t352: F, t3549: F, t3556: F, t6751: F, t837: F, t8481: F, t855: F) -> (F, F, F, F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t11302 = t1338 * t3552;
    let t11305 = t6755 * t1142;
    let t11310 = piecewise3::<F>(t332, t11216, F::cast_from(0.0_f64));
    let t11314 = t1348 * t3552;
    let t11319 = t6767 * t1142;
    let t11323 = piecewise5::<F>(t323, t1125 * t1307 * t330 + F::cast_from(2.0_f64) * t3517 * t837 * t330 + t11239 * t330 + t11244 * t330, t331, t11273 + t11298, -F::cast_from(0.63e1_f64) * t3556 * t8481 - F::cast_from(0.42e1_f64) * t11302 * t2438 - F::cast_from(0.945e1_f64) * t11305 * t8481 - F::cast_from(0.21e1_f64) * t3549 * t6751 - F::cast_from(0.105e1_f64) * t855 * t11310 * t352 - F::cast_from(0.315e1_f64) * t11314 * t2438 - F::cast_from(0.1575e1_f64) * t3556 * t6751 - F::cast_from(0.23625e1_f64) * t11319 * t8481);
    (t11302, t11305, t11310, t11314, t11319, t11323)
}
