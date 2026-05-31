//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1027/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1027<F: Float>(t322: F, t12828: F, t10533: F, t11305: F, t11319: F, t12348: F, t12355: F, t12683: F, t12849: F, t12851: F, t12854: F, t12856: F, t12883: F, t12908: F, t330: F, t352: F, t3549: F, t3556: F, t3675: F, t855: F) -> (F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t12918 = piecewise3::<F>(t332, t12828, F::cast_from(0.0_f64));
    let t12929 = piecewise5::<F>(t323, t12849 * t330 + F::cast_from(2.0_f64) * t12851 * t330 + t12854 * t330 + t12856 * t330, t331, t12883 + t12908, -F::cast_from(0.63e1_f64) * t3556 * t12683 - F::cast_from(0.42e1_f64) * t12348 * t3675 - F::cast_from(0.945e1_f64) * t11305 * t12683 - F::cast_from(0.21e1_f64) * t3549 * t10533 - F::cast_from(0.105e1_f64) * t855 * t12918 * t352 - F::cast_from(0.315e1_f64) * t12355 * t3675 - F::cast_from(0.1575e1_f64) * t3556 * t10533 - F::cast_from(0.23625e1_f64) * t11319 * t12683);
    (t12918, t12929)
}
