//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2156/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2156<F: Float>(t23508: F, t43292: F, t11013: F, t225: F, t10163: F, t386: F, t68: F, t11008: F, t3215: F, t3399: F, t3402: F, t11176: F, t300: F) -> (F, F, F, F, F, F, F, F) {
    let t43577 = t23508 * t43292;
    let t43599 = t11013 * t225;
    let t43603 = F::cast_from(1.0_f64) / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43619 = t11008 * t225;
    let t43636 = t3215 * t3215;
    let t43637 = F::cast_from(1.0_f64) / t43636;
    let t43688 = t3399 * t3399;
    let t43689 = F::cast_from(1.0_f64) / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = F::cast_from(1.0_f64) / t43691;
    let t43700 = t300 * t11176;
    (t43577, t43599, t43604, t43619, t43637, t43689, t43692, t43700)
}
