//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 503/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk503(t19: f64, t2230: f64, t601: f64, t604: f64, t84: f64, t85: f64, t24: f64) -> (f64, f64, f64, f64) {
    let t2232 = 0.9492e2_f64 * t19 * t2230;
    let t2235 = t601 * t604;
    let t2239 = 1.0_f64 / t85 / t84;
    let t2240 = t24 * t2239;
    (t2232, t2235, t2239, t2240)
}
