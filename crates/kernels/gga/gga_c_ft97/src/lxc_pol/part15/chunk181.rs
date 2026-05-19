//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 181/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk181<F: Float>(t137: F, t135: F, t416: F, t161: F, t376: F, t89: F, t143: F, t378: F) -> (F, F, F, F, F, F) {
    let t548 = t137 * t137;
    let t549 = F::new(1.0) / t548;
    let t550 = t135 * t549;
    let t552 = F::cast_from(0.16669500273148148149e-1_f64) * t416;
    let t568 = t89 * t376 * t161 / F::new(9.0);
    let t569 = t378 * t143;
    (t548, t549, t550, t552, t568, t569)
}
