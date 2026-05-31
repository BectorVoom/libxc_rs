//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 310/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk310<F: Float>(t12: F, t24: F, t1009: F, t83: F, t1008: F, t124: F, t207: F, t972: F, t1003: F, t333: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t1010 = t83 * t1009;
    let t1012 = F::cast_from(0.19751673498613801407e-1_f64) * t1008 * t124;
    let t1015 = piecewise3::<F>(t84, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t207 * t972);
    let t1018 = piecewise3::<F>(t90, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t333 * t1003);
    let t1020 = t1015 / F::cast_from(2.0_f64) + t1018 / F::cast_from(2.0_f64);
    (t1010, t1012, t1020)
}
