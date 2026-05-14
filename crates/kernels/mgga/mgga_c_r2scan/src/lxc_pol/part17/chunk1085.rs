//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1085/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1085<F: Float>(t11497: F, t12056: F, t3262: F, t11199: F, t12391: F, t11336: F, t37327: F, t43767: F, t31498: F, t3275: F, t3465: F, t12574: F, t39040: F, t12811: F, t498: F, t3352: F) -> (F, F, F, F, F, F, F) {
    let t44541 = 3.0 / 2.0 * t3262 * t12056 * t11497;
    let t44544 = 3.0 / 2.0 * t3262 * t11199 * t12391;
    let t44548 = 15.0 / 8.0 * t37327 * t11336 * t43767;
    let t44551 = t3275 * t3465 * t31498 / 4.0;
    let t44554 = 45.0 / 64.0 * t3275 * t39040 * t12574;
    let t44555 = t498 * t12811;
    let t44558 = t3275 * t44555 * t3352 / 4.0;
    (t44541, t44544, t44548, t44551, t44554, t44555, t44558)
}
