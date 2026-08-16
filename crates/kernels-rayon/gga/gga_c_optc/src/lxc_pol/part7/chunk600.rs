//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 600/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk600(t2936: f64, t2976: f64, t2843: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t389: f64, t1032: f64, t1036: f64, t1057: f64, t1035: f64, t385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2977 = t2936 * t2976;
    let t2980 = 0.23744444444444444444e-1_f64 * t2843;
    let t2985 = t2980 + 0.11872222222222222222e-1_f64 * t2845 - 0.11872222222222222222e-1_f64 * t2852 + 0.35616666666666666666e-1_f64 * t2858 - 0.17808333333333333333e-1_f64 * t2862;
    let t2987 = 0.62182e-1_f64 * t2985 * t389;
    let t2988 = t1032 * t1036;
    let t2990 = 2.0_f64 * t2988 * t1057;
    let t2991 = t1035 * t385;
    (t2977, t2985, t2987, t2988, t2990, t2991)
}
