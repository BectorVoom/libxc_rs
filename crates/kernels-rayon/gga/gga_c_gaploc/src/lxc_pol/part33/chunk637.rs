//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 637/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk637(t120: f64, t1461: f64, t586: f64, t1508: f64, t556: f64, t566: f64) -> (f64, f64, f64, f64) {
    let t4624 = t1461 * t120;
    let t4625 = t4624 * t586;
    let t4631 = t1508 * t556;
    let t4634 = t1461 * t566;
    (t4624, t4625, t4631, t4634)
}
