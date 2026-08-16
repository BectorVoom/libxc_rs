//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 815/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk815<F: Float>(t44: F, t6959: F, t2999: F, t4938: F, t1361: F, t3002: F, t1216: F, t4911: F, t1217: F, t2466: F, t415: F, t48: F, t3007: F, t4948: F, zeta_threshold: F) -> (F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t8560 = F::cast_from(0.21687162600603479684e-1_f64) * t6959;
    let t8561 = t4938 * t2999;
    let t8566 = t1361 * t3002;
    let t8571 = -F::cast_from(2.0_f64) * t1216 - F::cast_from(6.0_f64) * t4911;
    let t8575 = piecewise3::<F>(t45, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8561 * t415 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2466 * t1217 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8566 * t415 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t48 * t8571);
    let t8576 = t4948 * t3007;
    (t8560, t8571, t8575, t8576)
}
