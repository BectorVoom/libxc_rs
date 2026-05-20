//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3101/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101<F: Float>(t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F, t81423: F, t81425: F, t81427: F, t81429: F) -> F {
    let t81705 = -F::cast_from(0.5738611111111111111e0_f64) * t68262 - F::new(0.103295e1) * t68277 + F::new(0.104195e0) * t81423 - F::cast_from(0.69463333333333333333e-1_f64) * t81425 + F::cast_from(0.13892666666666666667e0_f64) * t81427 - F::new(0.41678e0) * t81429 + F::cast_from(0.69463333333333333333e-1_f64) * t68312 + F::cast_from(0.34431666666666666666e0_f64) * t68332 + F::cast_from(0.68863333333333333332e0_f64) * t68334 + F::cast_from(0.20658999999999999999e1_f64) * t68336 - F::new(0.41678e0) * t68368 - F::cast_from(0.9261777777777777778e-1_f64) * t68370;
    t81705
}
