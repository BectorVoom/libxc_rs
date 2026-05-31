//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 925/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk925<F: Float>(t28385: F, t7378: F, t28377: F, t7370: F, t18036: F, t5248: F, t7715: F, t11910: F, t11911: F, t28368: F, t1470: F, t18005: F, t24299: F, t24320: F, t24324: F, t24374: F, t24376: F, t2543: F, t28859: F, t28868: F, t28873: F, t28885: F, t6278: F, t725: F, t8915: F, t8923: F, t8927: F, t8931: F) -> F {
    let t29404 = t7378 * t28385;
    let t29416 = t7370 * t28377;
    let t29432 = t5248 * t18036 * t7715;
    let t29436 = t11910 * t11911 * t28368;
    let t29439 = F::cast_from(0.26531111111111111111e-1_f64) * t18005 + F::cast_from(0.15918666666666666666e0_f64) * t6278 * t29404 - F::cast_from(0.10612444444444444444e0_f64) * t24299 + F::cast_from(0.371475e-1_f64) * t2543 * t8927 - F::cast_from(0.232171875e-2_f64) * t725 * t28885 + F::cast_from(0.139303125e-1_f64) * t2543 * t8915 - F::cast_from(0.88437037037037037035e-1_f64) * t24320 - F::cast_from(0.79593333333333333333e-1_f64) * t24324 - F::cast_from(0.13265555555555555555e0_f64) * t6278 * t29416 - F::cast_from(0.619125e-2_f64) * t725 * t28868 + F::cast_from(0.27860625e-1_f64) * t2543 * t8923 - F::cast_from(0.1857375e-1_f64) * t2543 * t8931 + F::cast_from(0.9286875e-2_f64) * t725 * t28873 + F::cast_from(0.10612444444444444444e0_f64) * t24374 - F::cast_from(0.53062222222222222221e-1_f64) * t24376 - F::cast_from(0.371475e-1_f64) * t725 * t28859 - F::cast_from(0.13265555555555555556e0_f64) * t1470 * t29432 - F::cast_from(0.11791604938271604938e0_f64) * t1470 * t29436;
    t29439
}
