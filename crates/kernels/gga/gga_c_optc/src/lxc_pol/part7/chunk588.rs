//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 588/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk588<F: Float>(t1056: F, t2994: F, t2993: F, t2843: F, t2865: F, t2845: F, t2852: F, t2858: F, t2862: F, t2867: F, t2871: F, t2874: F, t2877: F, t2943: F, t2951: F, t2959: F, t2961: F) -> (F, F, F) {
    let t2995 = t2994 * t1056;
    let t2997 = 2.0 * t2993 * t2995;
    let t3000 = 0.39862222222222222223e0 * t2843;
    let t3007 = 0.13692777777777777778e0 * t2865;
    let t3012 = -0.9494625e0 * t2943 + 0.1898925e1 * t2951 + t3000 + 0.19931111111111111111e0 * t2845 - 0.19931111111111111111e0 * t2852 + 0.59793333333333333334e0 * t2858 - 0.29896666666666666667e0 * t2862 + 0.15358125e0 * t2959 + 0.3071625e0 * t2961 + t3007 + 0.10954222222222222222e0 * t2867 - 0.27385555555555555556e-1 * t2871 + 0.16431333333333333333e0 * t2874 - 0.82156666666666666667e-1 * t2877;
    (t2995, t2997, t3012)
}
