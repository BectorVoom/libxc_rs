//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1003/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1003(t11153: f64, t3439: f64, t9288: f64, t974: f64, t11147: f64, t11545: f64, t11660: f64, t1216: f64, t4582: f64, t10913: f64, t4987: f64, t3247: f64, t415: f64) -> (f64, f64, f64, f64, f64) {
    let t11759 = t3439 * t11153;
    let t11760 = t11759 * t9288;
    let t11761 = t974 * t11760;
    let t11764 = t11545 * t11147;
    let t11765 = t11764 * t9288;
    let t11766 = t974 * t11765;
    let t11769 = t11660 * t1216;
    let t11770 = t4582 * t11769;
    let t11773 = t4987 * t10913;
    let t11774 = t4582 * t11773;
    let t11778 = 1.0_f64 / t415 / t3247;
    (t11761, t11766, t11770, t11774, t11778)
}
