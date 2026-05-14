//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 150/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk150<F: Float>(t370: F, t432: F, t27: F, t89: F, t354: F, t366: F, t348: F, t104: F, t376: F, t5: F, t6: F) -> (F, F, F, F, F) {
    let t433 = t370 * t432;
    let t435 = t89 * t27 * t433;
    let t437 = -t354 - t366 / 18.0 - t435 / 6.0;
    let t438 = t348 * t437;
    let t442 = t89 * t376 * t104 / 9.0;
    let t443 = t5 * t6;
    (t433, t435, t438, t442, t443)
}
