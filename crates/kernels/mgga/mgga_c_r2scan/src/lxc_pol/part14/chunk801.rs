//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 801/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk801<F: Float>(t51: F, t4920: F, t893: F, t1224: F, t35: F, t1216: F, t476: F, t1225: F, t1228: F, t2517: F, t2520: F, t40: F, t6995: F, zeta_threshold: F) -> F {
    let t52 = t51 <= zeta_threshold;
    let t7073 = t4920 * t893;
    let t7076 = t1224 * t35;
    let t7081 = t476 * t1216;
    let t7086 = piecewise3::<F>(t52, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t7073 * t1225 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t7076 * t6995 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2517 * t1228 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7081 + F::cast_from(4.0_f64) * t2520 * t40);
    t7086
}
