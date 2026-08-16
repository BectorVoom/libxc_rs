//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 901/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk901(t124: f64, t9419: f64, t3192: f64, t574: f64, t1201: f64, t1390: f64) -> (f64, f64, f64, f64) {
    let t9420 = t9419 * t124;
    let t9421 = t9420 * t3192;
    let t9422 = t574 * t9421;
    let t9438 = t1201 * t124 * t1390;
    (t9420, t9421, t9422, t9438)
}
