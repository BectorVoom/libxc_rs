//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 805/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk805(t5754: f64, t732: f64, t5483: f64, t5496: f64, t5502: f64, t5583: f64, t5587: f64, t5736: f64, t5740: f64, t5744: f64, t5751: f64, t5753: f64) -> (f64, f64) {
    let t5756 = 0.17544670867903938621e1_f64 * t5754 * t732;
    let t5757 = -t5736 + t5740 - t5583 + t5587 - t5744 - t5751 - t5753 - t5756 - t5483 - t5496 + t5502;
    (t5756, t5757)
}
