//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 571/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk571(t218: f64, t219: f64, t3061: f64, t3026: f64, t334: f64, t2175: f64, t2212: f64, t2222: f64, t2224: f64, t3017: f64, t3028: f64, t3042: f64, t3047: f64, t3053: f64, t3055: f64, t3059: f64) -> (f64, f64, f64, f64) {
    let t3063 = t218 * t219 * t3061;
    let t3065 = t334 * t3026;
    let t3067 = t218 * t219 * t3065;
    let t3069 = -0.9494625e0_f64 * t3042 + 0.1898925e1_f64 * t3047 + t2212 - 0.29896666666666666667e0_f64 * t2175 - 0.29896666666666666667e0_f64 * t3017 + 0.8969e0_f64 * t3028 + 0.15358125e0_f64 * t3053 + 0.3071625e0_f64 * t3055 + t2222 - 0.16431333333333333333e0_f64 * t2224 - 0.16431333333333333333e0_f64 * t3059 + 0.24647e0_f64 * t3063 + 0.24647e0_f64 * t3067;
    (t3063, t3065, t3067, t3069)
}
