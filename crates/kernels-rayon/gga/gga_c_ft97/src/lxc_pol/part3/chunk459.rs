//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 459/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk459(t139: f64, t3379: f64, t527: f64, t1013: f64, t549: f64, t538: f64, t1014: f64, t542: f64, t133: f64, t135: f64, t2057: f64, t554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3380 = t139 * t3379;
    let t3381 = t527 * t3380;
    let t3383 = t549 * t1013;
    let t3384 = t3383 * t538;
    let t3387 = t542 * t1014;
    let t3392 = t133 * t135;
    let t3393 = t2057 * t1013;
    let t3394 = t3393 * t554;
    (t3380, t3381, t3384, t3387, t3392, t3393, t3394)
}
