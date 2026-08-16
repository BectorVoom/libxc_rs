//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 40/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk40(t135: f64, t139: f64, t35: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = 1.0_f64 <= zeta_threshold;
    let t140 = t135 * t139;
    let t141 = t35 * t35;
    let t142 = piecewise3(t34, t141, 1.0_f64);
    let t143 = t142 * t142;
    let t145 = 1.0_f64 / t143 / t142;
    (t140, t141, t142, t143, t145)
}
