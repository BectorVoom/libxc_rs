//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 655/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk655<F: Float>(t3046: F, t834: F, t1174: F, t2215: F, t836: F, t841: F, t1180: F, t218: F, t675: F, t1167: F, t824: F, t219: F, t3026: F, t334: F, t2175: F, t2212: F, t2222: F, t2224: F, t3017: F, t3028: F, t3042: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3047 = t834 * t3046;
    let t3052 = t2215 * t1174;
    let t3053 = t3052 * t836;
    let t3055 = t841 * t3046;
    let t3059 = t218 * t675 * t1180;
    let t3061 = t824 * t1167;
    let t3063 = t218 * t219 * t3061;
    let t3065 = t334 * t3026;
    let t3067 = t218 * t219 * t3065;
    let t3069 = -0.9494625e0 * t3042 + 0.1898925e1 * t3047 + t2212 - 0.29896666666666666667e0 * t2175 - 0.29896666666666666667e0 * t3017 + 0.8969e0 * t3028 + 0.15358125e0 * t3053 + 0.3071625e0 * t3055 + t2222 - 0.16431333333333333333e0 * t2224 - 0.16431333333333333333e0 * t3059 + 0.24647e0 * t3063 + 0.24647e0 * t3067;
    (t3047, t3052, t3053, t3055, t3059, t3061, t3063, t3065, t3067, t3069)
}
