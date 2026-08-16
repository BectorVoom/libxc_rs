//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1251/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1251(t1625: f64, t23592: f64, t1921: f64, t7577: f64, t14202: f64, t6765: f64, t14159: f64, t6717: f64, t23509: f64, t25682: f64, t25644: f64, t25650: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88138 = t23592 * t1625;
    let t88162 = t7577 * t1921;
    let t88321 = t6765 * t14202;
    let t88336 = t6717 * t14159;
    let t88342 = t23509 * t25682;
    let t88372 = t25650 * t25644;
    (t88138, t88162, t88321, t88336, t88342, t88372)
}
