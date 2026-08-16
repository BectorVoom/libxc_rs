//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1206/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1206(t2660: f64, t34123: f64, t7375: f64, t33312: f64, t3789: f64, t11449: f64, t11804: f64, t15843: f64, t190: f64, t2674: f64, t11522: f64, t15805: f64, t9799: f64) -> (f64, f64, f64, f64) {
    let t34125 = t2660 * t34123 * t7375;
    let t34127 = t33312 * t3789;
    let t34132 = t2674 * t190 * t11449 * t11804 * t15843;
    let t34135 = t15805 * t11522 * t9799;
    (t34125, t34127, t34132, t34135)
}
