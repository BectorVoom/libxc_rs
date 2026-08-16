//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 471/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk471(t1060: f64, t1629: f64, t1625: f64, t383: f64, t1058: f64, t1610: f64, t353: f64, t384: f64, t1055: f64, t1052: f64, t1604: f64, t1626: f64, t388: f64) -> (f64, f64, f64, f64, f64) {
    let t1630 = t1629 * t1060;
    let t1632 = t383 * t1625;
    let t1634 = t1058 * t1630 + t1610 * t384 + t1632 * t353;
    let t1635 = t1055 * t1634;
    let t1637 = -t1052 * t1635 + t1604 * t388 + t1626 * t388;
    (t1630, t1632, t1634, t1635, t1637)
}
