//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 587/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk587<F: Float>(t2936: F, t2976: F, t2843: F, t2845: F, t2852: F, t2858: F, t2862: F, t389: F, t1032: F, t1036: F, t1057: F, t1035: F, t385: F, t375: F, t1055: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2977 = t2936 * t2976;
    let t2980 = 0.23744444444444444444e-1 * t2843;
    let t2985 = t2980 + 0.11872222222222222222e-1 * t2845 - 0.11872222222222222222e-1 * t2852 + 0.35616666666666666666e-1 * t2858 - 0.17808333333333333333e-1 * t2862;
    let t2987 = 0.62182e-1 * t2985 * t389;
    let t2988 = t1032 * t1036;
    let t2990 = 2.0 * t2988 * t1057;
    let t2991 = t1035 * t385;
    let t2992 = 1.0 / t2991;
    let t2993 = t375 * t2992;
    let t2994 = t1055 * t1055;
    (t2977, t2985, t2987, t2988, t2990, t2991, t2992, t2993, t2994)
}
