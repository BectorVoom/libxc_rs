//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1364/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1364(t10525: f64, t10526: f64, t34239: f64, t6689: f64, t8411: f64, t31590: f64, t475: f64) -> (f64, f64, f64) {
    let t34318 = 0.42900587942220512002e1_f64 * t10525 * t10526 * t34239;
    let t34320 = 0.10725146985555128001e1_f64 * t8411 * t6689;
    let t34321 = t31590 * t475;
    (t34318, t34320, t34321)
}
