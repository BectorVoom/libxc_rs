//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 412/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk412(t5513: f64, t6427: f64, t5522: f64, t938: f64, t5540: f64, t5546: f64, t1701: f64, t5571: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6428 = t5513 * t6427;
    let t6431 = t5522 * t938;
    let t6434 = t5540 * t6427;
    let t6437 = t5546 * t938;
    let t6438 = t1701 * t6437;
    let t6441 = t5571 * t930;
    (t6428, t6431, t6434, t6437, t6438, t6441)
}
