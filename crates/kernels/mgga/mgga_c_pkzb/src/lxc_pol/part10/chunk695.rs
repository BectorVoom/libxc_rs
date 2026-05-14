//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 695/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk695<F: Float>(t218: F, t219: F, t3061: F, t3026: F, t334: F, t2175: F, t2212: F, t2222: F, t2224: F, t3017: F, t3028: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F) -> (F, F, F, F) {
    let t3063 = t218 * t219 * t3061;
    let t3065 = t334 * t3026;
    let t3067 = t218 * t219 * t3065;
    let t3069 = -0.9494625e0 * t3042 + 0.1898925e1 * t3047 + t2212 - 0.29896666666666666667e0 * t2175 - 0.29896666666666666667e0 * t3017 + 0.8969e0 * t3028 + 0.15358125e0 * t3053 + 0.3071625e0 * t3055 + t2222 - 0.16431333333333333333e0 * t2224 - 0.16431333333333333333e0 * t3059 + 0.24647e0 * t3063 + 0.24647e0 * t3067;
    (t3063, t3065, t3067, t3069)
}
