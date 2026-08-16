//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1901/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1901(t22635: f64, t26331: f64, t26337: f64, t90506: f64, t26216: f64, t81159: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t1385: f64, t22633: f64) -> (f64, f64, f64, f64) {
    let t90509 = t26331 * t22635 * t26337 * t90506;
    let t90511 = t81159 * t26216;
    let t90514 = t6897 * t794 * t26210;
    let t90516 = t1377 * t5187;
    let t90519 = t22633 * t22635 * t90516 * t1385;
    (t90509, t90511, t90514, t90519)
}
