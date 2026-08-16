//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 377/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk377(t653: f64, t667: f64, t1736: f64, t1743: f64, t1180: f64, t676: f64) -> (f64, f64, f64, f64) {
    let t1744 = t653 * t667;
    let t1745 = t1744 * t1736;
    let t1746 = t1743 * t1745;
    let t1749 = t676 * t1180;
    (t1744, t1745, t1746, t1749)
}
