//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 625/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk625(t1649: f64, t3679: f64, t1643: f64, t1629: f64, t188: f64, t116: f64, t205: f64, t1033: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3680 = t3679 * t1649;
    let t3681 = t1643 * t3680;
    let t3683 = t1629 * t188;
    let t3684 = t116 * t3683;
    let t3685 = t3684 * t205;
    let t3687 = t435 * t1033;
    (t3680, t3681, t3683, t3684, t3685, t3687)
}
