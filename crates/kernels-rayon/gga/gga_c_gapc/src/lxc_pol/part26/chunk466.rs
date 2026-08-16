//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 466/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk466(t1647: f64, t2580: f64, t597: f64, t818: f64, t906: f64, t871: f64, t897: f64, t1686: f64, t933: f64, t786: f64, t1086: f64, t1087: f64) -> (f64, f64, f64, f64, f64) {
    let t2581 = t1647 * t2580;
    let t2585 = t597 * t818 * t906;
    let t2588 = t871 * t897;
    let t2591 = t933 * t1686;
    let t2592 = t786 * t818;
    let t2594 = t1086 * t1087 * t2592;
    (t2581, t2585, t2588, t2591, t2594)
}
