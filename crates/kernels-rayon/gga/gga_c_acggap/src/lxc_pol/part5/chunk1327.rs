//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1327/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1327(t14712: f64, t14717: f64, t14718: f64, t14719: f64, t14720: f64, t2959: f64, t2961: f64, t2963: f64, t2966: f64, t5397: f64, t5401: f64, t6590: f64) -> f64 {
    let t24672 = -t14712 - 0.11696447245269292414e1_f64 * t2959 - 0.10389515463408878255e3_f64 * t2961 + 12.0_f64 * t5397 + 0.14649157844805236043e-2_f64 * t2963 - 0.36622894612013090108e-3_f64 * t2966 + t14717 - t14718 - t14719 + t14720 + 24.0_f64 * t6590 - 4.0_f64 * t5401;
    t24672
}
