//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1140/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1140(t33560: f64, t9419: f64, t11808: f64, t29516: f64, t3707: f64, t4780: f64, t2660: f64, t7375: f64, t33312: f64, t3789: f64, t11449: f64, t11804: f64, t15843: f64, t190: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34119 = t33560 * t9419;
    let t34121 = t11808 * t29516;
    let t34123 = t4780 * t3707;
    let t34125 = t2660 * t34123 * t7375;
    let t34127 = t33312 * t3789;
    let t34132 = t2674 * t190 * t11449 * t11804 * t15843;
    (t34119, t34121, t34123, t34125, t34127, t34132)
}
