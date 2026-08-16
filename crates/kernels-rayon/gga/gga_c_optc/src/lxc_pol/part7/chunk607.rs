//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 607/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk607(t3029: f64, t415: f64, t1081: f64, t1085: f64, t2916: f64, t406: f64, t1094: f64, t2917: f64, t2843: f64, t2865: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t2867: f64, t2871: f64, t2874: f64, t2877: f64, t2943: f64, t2951: f64, t2959: f64, t2961: f64) -> (f64, f64, f64, f64, f64) {
    let t3030 = t3029 * t415;
    let t3032 = t1081 * t1085;
    let t3035 = t406 * t2916;
    let t3036 = t2917 * t1094;
    let t3041 = 0.40256666666666666667e0_f64 * t2843;
    let t3048 = 0.137975e0_f64 * t2865;
    let t3053 = -0.1294625e1_f64 * t2943 + 0.258925e1_f64 * t2951 + t3041 + 0.20128333333333333334e0_f64 * t2845 - 0.20128333333333333333e0_f64 * t2852 + 0.60385e0_f64 * t2858 - 0.301925e0_f64 * t2862 + 0.82524375e-1_f64 * t2959 + 0.16504875e0_f64 * t2961 + t3048 + 0.11038e0_f64 * t2867 - 0.27595e-1_f64 * t2871 + 0.16557e0_f64 * t2874 - 0.82785e-1_f64 * t2877;
    (t3030, t3032, t3035, t3036, t3053)
}
