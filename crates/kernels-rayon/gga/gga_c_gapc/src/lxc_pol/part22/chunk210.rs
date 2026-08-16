//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 210/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk210(t257: f64, t748: f64, t105: f64, t107: f64, t260: f64, t269: f64, t438: f64, t446: f64, t447: f64, t751: f64) -> (f64, f64) {
    let t780 = t257 * t748;
    let t786 = 0.33843946638888888889e-3_f64 * t105 * t438 * t269 - 0.25382959979166666667e-3_f64 * t446 * t447 * t269 - 0.50765919958333333334e-3_f64 * t105 * t107 * t780 - 4.0_f64 * t260 * t751;
    (t780, t786)
}
