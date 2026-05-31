//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1250/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1250<F: Float>(t11497: F, t12056: F, t3262: F, t11199: F, t12391: F, t11336: F, t37327: F, t43767: F, t31498: F, t3275: F, t3465: F, t12574: F, t39040: F) -> (F, F, F, F, F) {
    let t44541 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t12056 * t11497;
    let t44544 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t11199 * t12391;
    let t44548 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37327 * t11336 * t43767;
    let t44551 = t3275 * t3465 * t31498 / F::cast_from(4.0_f64);
    let t44554 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t39040 * t12574;
    (t44541, t44544, t44548, t44551, t44554)
}
