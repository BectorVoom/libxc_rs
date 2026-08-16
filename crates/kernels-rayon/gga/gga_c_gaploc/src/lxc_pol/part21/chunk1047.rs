//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1047/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1047(t20368: f64, t20369: f64, t4786: f64, t6575: f64, t2293: f64, t447: f64, t1564: f64, t579: f64, t4390: f64, t4398: f64, t10524: f64, t1415: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20370 = t20368 * t20369;
    let t20374 = t4786 * t6575;
    let t20395 = t2293 * t447;
    let t20441 = t579 * t1564;
    let t20445 = t4398 * t4390;
    let t20471 = t1415 * t10524;
    (t20370, t20374, t20395, t20441, t20445, t20471)
}
