//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 558/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk558<F: Float>(t2665: F, t305: F, t140: F, t2748: F, t2670: F, t2672: F, t1: F, t313: F, t2650: F, t2701: F, t2704: F, t2708: F, t2712: F, t2716: F, t2721: F, t2725: F, t2729: F, t2731: F, t2734: F, t2737: F, t2740: F, t2745: F, t314: F, t324: F, t899: F, t917: F, t930: F, t943: F, t953: F) -> (F, F, F, F, F, F) {
    let t2749 = t305 * t2665;
    let t2750 = t2749 * t140;
    let t2751 = t2748 * t2750;
    let t2752 = t2670 * t2672;
    let t2753 = t2752 * t1;
    let t2754 = t313 * t2753;
    let t2757 = -F::cast_from(0.10076140891672839458e-1_f64) * t953 * t2650 - F::cast_from(0.57954409931925052364e-1_f64) * t930 * t2701 - F::cast_from(0.5373941808892181044e-1_f64) * t2704 * t899 - F::cast_from(0.6237918122117623248e2_f64) * t2708 * t943 - F::cast_from(0.60587206808032502059e1_f64) * t2712 * t917 + F::cast_from(0.75734008510040627574e0_f64) * t2716 + F::cast_from(0.75734008510040627574e0_f64) * t2721 * t2725 - t2729 + F::cast_from(0.25526223592237859959e0_f64) * t2731 * t314 - F::cast_from(0.5373941808892181044e-1_f64) * t2734 + F::cast_from(0.84999801233490076802e0_f64) * t2737 * t324 - F::cast_from(0.15454509315180013964e0_f64) * t2740 - t2745 + F::cast_from(0.23229342182245570105e2_f64) * t2751 * t2754;
    (t2750, t2751, t2752, t2753, t2754, t2757)
}
