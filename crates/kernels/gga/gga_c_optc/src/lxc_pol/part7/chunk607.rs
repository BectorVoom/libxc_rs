//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 607/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk607<F: Float>(t3029: F, t415: F, t1081: F, t1085: F, t2916: F, t406: F, t1094: F, t2917: F, t2843: F, t2865: F, t2845: F, t2852: F, t2858: F, t2862: F, t2867: F, t2871: F, t2874: F, t2877: F, t2943: F, t2951: F, t2959: F, t2961: F) -> (F, F, F, F, F) {
    let t3030 = t3029 * t415;
    let t3032 = t1081 * t1085;
    let t3035 = t406 * t2916;
    let t3036 = t2917 * t1094;
    let t3041 = F::cast_from(0.40256666666666666667e0_f64) * t2843;
    let t3048 = F::cast_from(0.137975e0_f64) * t2865;
    let t3053 = -F::cast_from(0.1294625e1_f64) * t2943 + F::cast_from(0.258925e1_f64) * t2951 + t3041 + F::cast_from(0.20128333333333333334e0_f64) * t2845 - F::cast_from(0.20128333333333333333e0_f64) * t2852 + F::cast_from(0.60385e0_f64) * t2858 - F::cast_from(0.301925e0_f64) * t2862 + F::cast_from(0.82524375e-1_f64) * t2959 + F::cast_from(0.16504875e0_f64) * t2961 + t3048 + F::cast_from(0.11038e0_f64) * t2867 - F::cast_from(0.27595e-1_f64) * t2871 + F::cast_from(0.16557e0_f64) * t2874 - F::cast_from(0.82785e-1_f64) * t2877;
    (t3030, t3032, t3035, t3036, t3053)
}
