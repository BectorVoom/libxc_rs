//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1402/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1402(t10504: f64, t138: f64, t2438: f64, t2828: f64, t11044: f64, t11050: f64, t11015: f64, t2461: f64, t11010: f64, t689: f64, t779: f64, t2769: f64, t786: f64, t861: f64) -> (f64, f64, f64, f64, f64) {
    let t41056 = t10504 * t138 * t2438 * t2828;
    let t41058 = t11044 * t11050;
    let t41060 = t2461 * t11015;
    let t41063 = t689 * t779 * t11010;
    let t41066 = t786 * t861 * t2769;
    (t41056, t41058, t41060, t41063, t41066)
}
