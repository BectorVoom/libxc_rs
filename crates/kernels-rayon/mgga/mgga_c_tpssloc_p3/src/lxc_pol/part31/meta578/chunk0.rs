//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1814/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814(t26339: f64, t81159: f64, t22716: f64, t7697: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90500 = t81159 * t26339;
    let t90503 = t22716 * t7697;
    let t90511 = t81159 * t26216;
    let t90514 = t6897 * t794 * t26210;
    let t90516 = t1377 * t5187;
    let t90521 = t81186 * t7692;
    (t90500, t90503, t90511, t90514, t90516, t90521)
}
