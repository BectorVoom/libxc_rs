//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 583/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk583(t2756: f64, t999: f64, t3518: f64, t535: f64, t3560: f64, t448: f64, t203: f64, t3529: f64) -> (f64, f64, f64, f64) {
    let t11157 = t999 * t2756;
    let t11160 = t535 * t3518;
    let t11163 = t3560 * t448;
    let t11167 = t203 * t3529;
    (t11157, t11160, t11163, t11167)
}
