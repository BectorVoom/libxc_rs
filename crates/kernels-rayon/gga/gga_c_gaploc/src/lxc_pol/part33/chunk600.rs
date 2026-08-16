//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 600/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk600(t1960: f64, t3459: f64, t3040: f64, t955: f64, t2976: f64, t959: f64, t1645: f64, t948: f64) -> (f64, f64, f64, f64) {
    let t3461 = 2.0_f64 * t1960 * t3459;
    let t3463 = 0.35750489951850426669e0_f64 * t955 * t3040;
    let t3468 = t2976 * t959;
    let t3469 = 0.14896037479937677779e-1_f64 * t3468;
    let t3470 = t1645 * t948;
    (t3461, t3463, t3469, t3470)
}
