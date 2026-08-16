//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1082/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1082(t14212: f64, t607: f64, t360: f64, t883: f64, t1022: f64, t10632: f64, t2906: f64, t11066: f64, t3040: f64, t6739: f64, t135: f64, t457: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14213 = t14212 * t607;
    let t14219 = t360 * t883;
    let t14220 = t14219 * t607;
    let t14227 = t607 * t1022;
    let t14228 = t14227 * t360;
    let t14259 = t10632 * t2906;
    let t14590 = t11066 * t3040;
    let t14630 = t6739 * t3040 * t360;
    let t15281 = t135 * t457;
    (t14213, t14220, t14228, t14259, t14590, t14630, t15281)
}
