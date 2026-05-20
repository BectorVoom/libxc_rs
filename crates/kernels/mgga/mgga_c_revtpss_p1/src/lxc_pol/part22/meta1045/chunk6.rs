//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3665/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3665<F: Float>(t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t68332: F, t68334: F, t68336: F) -> F {
    let t69246 = -F::cast_from(0.34431666666666666667e0_f64) * t68287 - F::new(0.20659e1) * t68292 + F::new(0.20659e1) * t68297 + F::new(0.103295e1) * t68301 + F::new(0.309885e1) * t68305 - F::cast_from(0.15302962962962962963e1_f64) * t68310 + F::cast_from(0.46308888888888888889e-1_f64) * t68312 + F::new(0.41678e0) * t68315 + F::new(0.62517e0) * t68319 + F::new(0.250068e1) * t68322 - F::new(0.20839e0) * t68326 - F::new(0.125034e1) * t68330 + F::cast_from(0.22954444444444444444e0_f64) * t68332 + F::cast_from(0.45908888888888888889e0_f64) * t68334 + F::cast_from(0.13772666666666666666e1_f64) * t68336;
    t69246
}
