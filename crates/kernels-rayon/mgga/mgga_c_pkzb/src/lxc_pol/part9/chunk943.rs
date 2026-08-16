//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 943/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk943(t1107: f64, t5493: f64, t1956: f64, t1084: f64, t1856: f64, t1899: f64, t1100: f64, t1976: f64, t1088: f64, t1937: f64, t1079: f64, t1878: f64, t218: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7308 = t1107 * t5493;
    let t7309 = t7308 * t1956;
    let t7312 = t1084 * t1856;
    let t7314 = 6.0_f64 * t1899 * t7312;
    let t7315 = t1100 * t1976;
    let t7324 = t1088 * t1937;
    let t7332 = t218 * t1878 * t1079;
    (t7308, t7309, t7312, t7314, t7315, t7324, t7332)
}
