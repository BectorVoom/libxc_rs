//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3111/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3111<F: Float>(t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F, t81423: F, t81425: F, t81427: F, t81429: F) -> F {
    let t81931 = -F::cast_from(0.33547222222222222222e0_f64) * t68262 - F::cast_from(0.60385000000000000002e0_f64) * t68277 + F::cast_from(0.82785e-1_f64) * t81423 - F::cast_from(0.5519e-1_f64) * t81425 + F::cast_from(0.11038e0_f64) * t81427 - F::cast_from(0.33114e0_f64) * t81429 + F::cast_from(0.5519e-1_f64) * t68312 + F::cast_from(0.20128333333333333334e0_f64) * t68332 + F::cast_from(0.40256666666666666666e0_f64) * t68334 + F::cast_from(0.12077e1_f64) * t68336 - F::cast_from(0.33114e0_f64) * t68368 - F::cast_from(0.73586666666666666666e-1_f64) * t68370;
    t81931
}
