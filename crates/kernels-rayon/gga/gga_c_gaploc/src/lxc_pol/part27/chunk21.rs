//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 21/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk21(t46: f64, t55: f64, t44: f64, t2: f64, t3: f64) -> (f64, f64, f64, f64) {
    let t56 = t46 * t55;
    let t58 = 0.19751789702565206229e-1_f64 * t44 * t56;
    let t59 = t3 * t2;
    let t60 = 1.0_f64 / t59;
    (t56, t58, t59, t60)
}
