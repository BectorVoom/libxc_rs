//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1304/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1304(t10169: f64, t11180: f64, t18520: f64, t898: f64, t31333: f64, t31335: f64, t31337: f64, t31339: f64, t31369: f64, t31372: f64, t31375: f64, t31377: f64, t31380: f64, t31383: f64) -> (f64, f64) {
    let t31640 = 0.12304822629859687989e5_f64 * t898 * t18520 * t11180 * t10169;
    let t31641 = t31333 - t31335 + t31337 + t31339 - t31369 - t31372 - t31375 + t31377 + t31380 + t31383 + t31640;
    (t31640, t31641)
}
