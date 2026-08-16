//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1034/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1034(t1985: f64, t26193: f64, t33296: f64, t127430: f64, t22633: f64, t22635: f64, t31558: f64, t122124: f64, t1799: f64, t1992: f64, t26989: f64, t6439: f64) -> (f64, f64, f64, f64) {
    let t128797 = t1985 * t26193 * t33296;
    let t128805 = t22633 * t22635 * t31558 * t127430;
    let t128809 = t22633 * t22635 * t122124 * t1799;
    let t128816 = t1992 * t22635 * t26989 * t6439;
    (t128797, t128805, t128809, t128816)
}
