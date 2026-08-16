//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1273/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1273<F: Float>(t24: F, t10523: F, t10528: F, t1430: F, t18408: F, t2179: F, t28895: F, t28898: F, t28906: F, t3019: F, t507: F, t7932: F, t7935: F, t821: F, t8742: F, t9784: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t31086 = piecewise3::<F>(t90, F::cast_from(0.0_f64), F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t18408 * t10523 * t507 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t9784 * t1430 - F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t7932 * t28895 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t7935 * t28898 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3019 * t8742 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2179 * t10528 * t507 - t821 * t28906 / F::cast_from(3.0_f64));
    t31086
}
