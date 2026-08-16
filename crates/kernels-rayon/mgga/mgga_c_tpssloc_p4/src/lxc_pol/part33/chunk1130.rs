//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1130/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1130(t4571: f64, t6765: f64, t4630: f64, t6755: f64, t1036: f64, t7586: f64, t1409: f64, t1933: f64, t1937: f64, t1597: f64, t40: f64, t23479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25616 = t6765 * t4571;
    let t25618 = t6755 * t4630;
    let t25625 = t7586 * t1036;
    let t25628 = t1933 * t1409;
    let t25629 = t25628 * t1937;
    let t25637 = t40 * t1597;
    let t25638 = t1933 * t25637;
    let t25639 = t25638 * t23479;
    (t25616, t25618, t25625, t25629, t25637, t25639)
}
