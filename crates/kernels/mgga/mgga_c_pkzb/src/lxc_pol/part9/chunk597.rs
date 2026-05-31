//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 597/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk597<F: Float>(t12: F, t24: F, t124: F, t2557: F, t207: F, t8: F, t1064: F, t1429: F, t439: F, t333: F, t1165: F, t507: F, zeta_threshold: F) -> (F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t2559 = F::cast_from(0.19751673498613801407e-1_f64) * t2557 * t124;
    let t2562 = t207 * t8;
    let t2566 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1064 * t439 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2562 * t1429);
    let t2569 = t333 * t8;
    let t2573 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1165 * t507 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2569 * t1429);
    let t2575 = t2566 / F::cast_from(2.0_f64) + t2573 / F::cast_from(2.0_f64);
    (t2559, t2562, t2569, t2575)
}
