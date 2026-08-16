//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 602/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk602(t1056: f64, t2994: f64, t2993: f64, t2843: f64, t2865: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t2867: f64, t2871: f64, t2874: f64, t2877: f64, t2943: f64, t2951: f64, t2959: f64, t2961: f64) -> (f64, f64, f64) {
    let t2995 = t2994 * t1056;
    let t2997 = 2.0_f64 * t2993 * t2995;
    let t3000 = 0.39862222222222222223e0_f64 * t2843;
    let t3007 = 0.13692777777777777778e0_f64 * t2865;
    let t3012 = -0.9494625e0_f64 * t2943 + 0.1898925e1_f64 * t2951 + t3000 + 0.19931111111111111111e0_f64 * t2845 - 0.19931111111111111111e0_f64 * t2852 + 0.59793333333333333334e0_f64 * t2858 - 0.29896666666666666667e0_f64 * t2862 + 0.15358125e0_f64 * t2959 + 0.3071625e0_f64 * t2961 + t3007 + 0.10954222222222222222e0_f64 * t2867 - 0.27385555555555555556e-1_f64 * t2871 + 0.16431333333333333333e0_f64 * t2874 - 0.82156666666666666667e-1_f64 * t2877;
    (t2995, t2997, t3012)
}
