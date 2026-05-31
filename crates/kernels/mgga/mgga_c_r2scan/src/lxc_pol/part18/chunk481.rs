//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 481/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk481<F: Float>(t44: F, t51: F, t1216: F, t2509: F, t2512: F, t415: F, t1224: F, t893: F, t35: F, t476: F, t419: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2516 = piecewise3::<F>(t45, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2509 * t415 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2512 * t1216);
    let t2517 = t1224 * t893;
    let t2520 = t476 * t35;
    let t2524 = piecewise3::<F>(t52, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2517 * t419 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2520 * t1216);
    let t2526 = t2516 / F::cast_from(2.0_f64) + t2524 / F::cast_from(2.0_f64);
    (t2517, t2526)
}
