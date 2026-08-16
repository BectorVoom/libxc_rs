//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1311/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1311(t18657: f64, t2380: f64, t8376: f64, t19107: f64, t22971: f64, t19109: f64, t6460: f64, t19116: f64, t6517: f64, t3185: f64, t6475: f64, t8350: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23061 = t2380 * t18657 * t8376;
    let t23075 = t19107 * t22971;
    let t23076 = t19109 * t6460;
    let t23081 = t19116 * t22971;
    let t23082 = t6517 * t6460;
    let t23088 = t3185 * t6475 * t8350;
    (t23061, t23075, t23076, t23081, t23082, t23088)
}
