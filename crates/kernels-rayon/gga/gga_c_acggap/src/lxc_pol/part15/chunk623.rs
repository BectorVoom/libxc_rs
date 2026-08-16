//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 623/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk623(t1089: f64, t368: f64, t5959: f64, t1487: f64, t495: f64, t1734: f64, t322: f64, t175: f64, t384: f64, t1426: f64, t5651: f64, t1817: f64, t3343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5961 = t1089 * t368 * t5959;
    let t5964 = t495 * t1487;
    let t5966 = t1089 * t368 * t5964;
    let t5969 = t1734 * t322;
    let t5971 = t1089 * t175 * t5969;
    let t5972 = t384 * t5971;
    let t5975 = t1426 * t175 * t5651;
    let t5978 = t3343 * t1817;
    (t5961, t5964, t5966, t5969, t5971, t5972, t5975, t5978)
}
