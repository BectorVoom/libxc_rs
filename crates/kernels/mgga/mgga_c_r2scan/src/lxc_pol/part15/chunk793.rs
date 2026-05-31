//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 793/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk793<F: Float>(t44: F, t1361: F, t35: F, t1216: F, t415: F, t1213: F, t1219: F, t2466: F, t2469: F, t40: F, t48: F, t6976: F, t4948: F, t893: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t6979 = t1361 * t35;
    let t6980 = t1216 * t415;
    let t6990 = piecewise3::<F>(t45, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t6976 * t1213 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6979 * t6980 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2466 * t1219 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t48 * t1216 - F::cast_from(8.0_f64) * t2469 * t40);
    let t6991 = t4948 * t893;
    (t6980, t6990, t6991)
}
