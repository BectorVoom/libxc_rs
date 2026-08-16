//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 558/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk558(t2665: f64, t305: f64, t140: f64, t2748: f64, t2670: f64, t2672: f64, t1: f64, t313: f64, t2650: f64, t2701: f64, t2704: f64, t2708: f64, t2712: f64, t2716: f64, t2721: f64, t2725: f64, t2729: f64, t2731: f64, t2734: f64, t2737: f64, t2740: f64, t2745: f64, t314: f64, t324: f64, t899: f64, t917: f64, t930: f64, t943: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2749 = t305 * t2665;
    let t2750 = t2749 * t140;
    let t2751 = t2748 * t2750;
    let t2752 = t2670 * t2672;
    let t2753 = t2752 * t1;
    let t2754 = t313 * t2753;
    let t2757 = -0.10076140891672839458e-1_f64 * t953 * t2650 - 0.57954409931925052364e-1_f64 * t930 * t2701 - 0.5373941808892181044e-1_f64 * t2704 * t899 - 0.6237918122117623248e2_f64 * t2708 * t943 - 0.60587206808032502059e1_f64 * t2712 * t917 + 0.75734008510040627574e0_f64 * t2716 + 0.75734008510040627574e0_f64 * t2721 * t2725 - t2729 + 0.25526223592237859959e0_f64 * t2731 * t314 - 0.5373941808892181044e-1_f64 * t2734 + 0.84999801233490076802e0_f64 * t2737 * t324 - 0.15454509315180013964e0_f64 * t2740 - t2745 + 0.23229342182245570105e2_f64 * t2751 * t2754;
    (t2750, t2751, t2752, t2753, t2754, t2757)
}
