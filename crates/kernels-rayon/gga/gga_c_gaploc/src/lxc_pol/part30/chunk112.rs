//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 112/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk112(t70: f64, t71: f64, t64: f64, t1: f64, t341: f64, t67: f64, t345: f64, t347: f64, t14: f64, t344: f64) -> (f64, f64, f64, f64, f64) {
    let t386 = 1.0_f64 / t71 / t70;
    let t387 = t64 * t386;
    let t389 = t341 * t67 * t1;
    let t394 = -0.66066666666666666667e-2_f64 * t345 - 0.41275e-2_f64 * t347;
    let t397 = -t389 * t344 / 12.0_f64 + t14 * t394 / 2.0_f64;
    (t386, t387, t389, t394, t397)
}
