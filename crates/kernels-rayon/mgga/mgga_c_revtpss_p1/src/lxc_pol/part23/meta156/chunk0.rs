//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 954/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk954(t4424: f64, t827: f64, t828: f64, t1559: f64, t221: f64, t2485: f64, t2484: f64, t1544: f64, t775: f64) -> (f64, f64, f64, f64) {
    let t4426 = t827 * t828 * t4424;
    let t4430 = t2485 * t221 * t1559;
    let t4431 = t2484 * t4430;
    let t4433 = t1544 * t775;
    (t4426, t4430, t4431, t4433)
}
