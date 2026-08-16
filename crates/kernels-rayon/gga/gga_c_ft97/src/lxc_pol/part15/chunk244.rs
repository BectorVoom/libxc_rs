//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 244/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk244(t140: f64, t1013: f64, t550: f64, t133: f64, t1010: f64) -> (f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t1014 = t550 * t1013;
    let t1015 = t133 * t1014;
    let t1017 = piecewise3(t141, 2.0_f64 * t1010 - t1015, 0.0_f64);
    (t1014, t1017)
}
