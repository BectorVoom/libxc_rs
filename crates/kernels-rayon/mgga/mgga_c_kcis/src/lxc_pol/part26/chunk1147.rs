//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1147/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1147(t7091: f64, t7899: f64, t6176: f64, t553: f64, t7262: f64, t303: f64, t7203: f64, t6923: f64, t21484: f64, t7909: f64, t3984: f64, t27340: f64, t7042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29299 = t7899 * t7091;
    let t29300 = t6176 * t29299;
    let t29304 = t553 * t7262;
    let t29305 = t303 * t29304;
    let t29307 = t553 * t7203;
    let t29308 = t303 * t29307;
    let t29310 = t553 * t6923;
    let t29311 = t303 * t29310;
    let t29313 = t7909 * t21484;
    let t29314 = t3984 * t29313;
    let t29323 = t27340 * t7042;
    (t29299, t29300, t29304, t29305, t29307, t29308, t29310, t29311, t29313, t29314, t29323)
}
