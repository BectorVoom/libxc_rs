//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 632/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk632(t1078: f64, t3757: f64, t128: f64, t2206: f64, t1033: f64, t311: f64, t3297: f64, t2580: f64, t3679: f64, t2578: f64, t188: f64, t2566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3758 = t3757 * t1078;
    let t3760 = t2206 * t128;
    let t3761 = t3760 * t1033;
    let t3763 = t311 * t3761 * t3297;
    let t3765 = t3679 * t2580;
    let t3766 = t2578 * t3765;
    let t3768 = t2566 * t188;
    (t3758, t3760, t3761, t3763, t3765, t3766, t3768)
}
