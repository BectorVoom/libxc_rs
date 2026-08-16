//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2591/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2591(t1409: f64, t3450: f64, t3469: f64, t15288: f64, t15338: f64, t3447: f64, t11583: f64, t12652: f64, t12648: f64, t11570: f64, t14165: f64, t44607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52170 = t3450 * t1409 * t3469;
    let t52191 = t3447 * t15338 * t15288;
    let t52216 = t11583 * t12652;
    let t52220 = t11583 * t12648;
    let t52224 = t11570 * t14165;
    let t52228 = t44607 * t14165;
    (t52170, t52191, t52216, t52220, t52224, t52228)
}
