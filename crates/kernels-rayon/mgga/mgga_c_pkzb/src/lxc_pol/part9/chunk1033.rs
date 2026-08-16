//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1033/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1033(t8322: f64, t8371: f64, t8405: f64, t8478: f64, t158: f64, t1255: f64, t2429: f64, t6546: f64, t2428: f64, t3278: f64, t951: f64, t2453: f64, t3254: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8480 = t8322 + t8371 + t8405 + t8478;
    let t8481 = t8480 * t158;
    let t8497 = t6546 * t1255 * t2429;
    let t8500 = t2428 * t3278;
    let t8501 = t8500 * t951;
    let t8504 = t3254 * t2453;
    (t8480, t8481, t8497, t8500, t8501, t8504)
}
