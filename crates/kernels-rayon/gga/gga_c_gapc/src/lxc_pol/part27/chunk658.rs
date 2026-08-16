//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 658/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk658(t1642: f64, t561: f64, t116: f64, t4978: f64, t188: f64, t3137: f64, t186: f64, t424: f64) -> (f64, f64, f64, f64) {
    let t5252 = t561 * t1642;
    let t5260 = t116 * t4978;
    let t5261 = t3137 * t188;
    let t5285 = t424 * t186;
    (t5252, t5260, t5261, t5285)
}
