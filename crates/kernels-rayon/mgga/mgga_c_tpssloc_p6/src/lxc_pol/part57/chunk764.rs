//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 764/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk764(t28107: f64, t553: f64, t1998: f64, t6434: f64, t214: f64, t1985: f64, t19739: f64, t550: f64, t6976: f64, t1992: f64, t19660: f64, t22709: f64, t6388: f64) -> (f64, f64, f64, f64, f64) {
    let t28156 = t553 * t28107;
    let t28159 = t1998 * t6434;
    let t28160 = t214 * t28159;
    let t28161 = t1985 * t28160;
    let t28163 = t19739 * t550;
    let t28164 = t6976 * t28163;
    let t28165 = t1992 * t28164;
    let t28167 = t19660 * t550;
    let t28168 = t6976 * t28167;
    let t28169 = t1992 * t28168;
    let t28171 = t22709 * t6388;
    (t28156, t28161, t28165, t28169, t28171)
}
