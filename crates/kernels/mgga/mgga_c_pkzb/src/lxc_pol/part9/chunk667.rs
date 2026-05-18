//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 667/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk667<F: Float>(t3026: F, t334: F, t218: F, t219: F, t2175: F, t2212: F, t2222: F, t2224: F, t3017: F, t3028: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F, t3063: F) -> (F, F, F) {
    let t3065 = t334 * t3026;
    let t3067 = t218 * t219 * t3065;
    let t3069 = -F::new(0.9494625e0) * t3042 + F::new(0.1898925e1) * t3047 + t2212 - F::new(0.29896666666666666667e0) * t2175 - F::new(0.29896666666666666667e0) * t3017 + F::new(0.8969e0) * t3028 + F::new(0.15358125e0) * t3053 + F::new(0.3071625e0) * t3055 + t2222 - F::new(0.16431333333333333333e0) * t2224 - F::new(0.16431333333333333333e0) * t3059 + F::new(0.24647e0) * t3063 + F::new(0.24647e0) * t3067;
    (t3065, t3067, t3069)
}
