//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 527/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk527<F: Float>(t51: F, t1216: F, t2474: F, t2477: F, t419: F, t2473: F, zeta_threshold: F) -> F {
    let t52 = t51 <= zeta_threshold;
    let t2481 = piecewise3::<F>(t52, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2474 * t419 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2477 * t1216);
    let t2482 = t2473 + t2481;
    t2482
}
