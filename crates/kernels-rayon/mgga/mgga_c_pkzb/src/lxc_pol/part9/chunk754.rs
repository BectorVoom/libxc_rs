//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 754/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk754(t1727: f64, t1756: f64, t4867: f64, t4870: f64, t4873: f64, t4876: f64, t4879: f64, t4881: f64, t4884: f64, t4887: f64, t5077: f64, t5079: f64, t5081: f64, t5087: f64) -> (f64, f64) {
    let t5315 = t1727 * t1756;
    let t5317 = t4867 + t4870 + t4873 - t4876 - t4879 + t4881 + t4884 + t4887 + t5077 + t5079 - t5081 - t5087;
    (t5315, t5317)
}
