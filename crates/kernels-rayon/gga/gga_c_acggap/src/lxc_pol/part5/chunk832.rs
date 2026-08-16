//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 832/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk832(t132: f64, t2808: f64, t2810: f64, t721: f64, t200: f64, t220: f64, t328: f64, t123: f64, t759: f64, t762: f64, t2604: f64, t704: f64) -> (f64, f64, f64, f64) {
    let t11578 = 0.68734380377411894876e1_f64 * t721 * t132 * t2808 * t2810;
    let t11582 = 0.22161481481481481481e0_f64 * t721 * t328 * t200 * t220;
    let t11586 = 0.28493333333333333333e0_f64 * t721 * t123 * t759 * t762;
    let t11591 = t704 * t2604;
    (t11578, t11582, t11586, t11591)
}
