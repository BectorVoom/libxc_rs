//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 686/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk686(t4883: f64, t1625: f64, t512: f64, t83: f64, t1497: f64, t542: f64, t1507: f64, t540: f64, t51: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4884 = 240.0_f64 * t4883;
    let t4885 = t512 * t1625;
    let t4886 = t83 * t4885;
    let t4888 = t542 * t1497;
    let t4892 = t1497 * t1507 * t540;
    let t4902 = t51 * t574;
    (t4884, t4885, t4886, t4888, t4892, t4902)
}
