//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 583/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk583<F: Float>(t1038: F, t2950: F, t2843: F, t373: F, t2942: F, t1045: F, t2865: F, t2845: F, t2852: F, t2858: F, t2862: F, t2867: F, t2871: F, t2874: F, t2877: F, t2943: F) -> (F, F, F, F, F) {
    let t2951 = t1038 * t2950;
    let t2953 = 0.68863333333333333333e0 * t2843;
    let t2958 = 1.0/f64::sqrt(t373);
    let t2959 = t2958 * t2942;
    let t2961 = t1045 * t2950;
    let t2963 = 0.17365833333333333333e0 * t2865;
    let t2968 = -0.17648625e1 * t2943 + 0.3529725e1 * t2951 + t2953 + 0.34431666666666666666e0 * t2845 - 0.34431666666666666667e0 * t2852 + 0.103295e1 * t2858 - 0.516475e0 * t2862 + 0.31558125e0 * t2959 + 0.6311625e0 * t2961 + t2963 + 0.13892666666666666667e0 * t2867 - 0.34731666666666666667e-1 * t2871 + 0.20839e0 * t2874 - 0.104195e0 * t2877;
    (t2951, t2958, t2959, t2961, t2968)
}
