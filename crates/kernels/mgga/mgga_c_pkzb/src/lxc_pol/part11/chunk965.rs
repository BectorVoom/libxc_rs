//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 965/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk965<F: Float>(t12: F, t24: F, t10513: F, t10518: F, t1064: F, t1837: F, t207: F, t3366: F, t10523: F, t10528: F, t1165: F, t2179: F, t333: F, t3374: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t10546 = piecewise3::<F>(t84, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1837 * t10513 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1064 * t3366 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t207 * t10518);
    let t10554 = piecewise3::<F>(t90, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2179 * t10523 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1165 * t3374 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t333 * t10528);
    (t10546, t10554)
}
