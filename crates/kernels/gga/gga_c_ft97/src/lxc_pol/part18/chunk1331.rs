//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1331/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1331<F: Float>(t27087: F, t376: F, t5890: F, t105512: F, t446: F, t9073: F, t27174: F, t89: F, t24: F, t9236: F, t23657: F, t23658: F, t6630: F, t27114: F, t375: F, t105329: F, t27: F, t526: F) -> (F, F, F, F, F, F, F, F, F) {
    let t105671 = t5890 * t376 * t27087;
    let t105672 = t105671 / 6.0;
    let t105674 = t446 * t9073 * t105512;
    let t105677 = t89 * t376 * t27174;
    let t105678 = 4.0 / 3.0 * t105677;
    let t105679 = t24 * t9236;
    let t105682 = t23657 * t105679 * t6630 * t23658;
    let t105685 = t89 * t375 * t27114;
    let t105686 = 2.0 / 3.0 * t105685;
    let t105689 = t89 * t27 * t526 * t105329;
    (t105671, t105672, t105674, t105677, t105678, t105682, t105685, t105686, t105689)
}
