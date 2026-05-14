//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1219/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1219<F: Float>(t100477: F, t100479: F, t100481: F, t100466: F, t100469: F, t100472: F, t100475: F, t101576: F, t93415: F, t93422: F, t93424: F, t93450: F, t101587: F, t101595: F, t101598: F, t101615: F) -> (F, F, F, F, F) {
    let t102164 = 2.0 / 9.0 * t100477;
    let t102165 = 2.0 / 9.0 * t100479;
    let t102166 = 2.0 / 27.0 * t100481;
    let t102169 = -2.0 / 3.0 * t100466 - t100469 / 3.0 - 2.0 / 3.0 * t100472 + t100475 / 9.0 + t102164 + t102165 - t102166 - t101576 / 2.0 - t93415 - t93422 - 4.0 / 9.0 * t93424 - t93450;
    let t102173 = t101587 / 6.0;
    let t102175 = 2.0 / 3.0 * t101595;
    let t102176 = t101598 / 9.0;
    let t102181 = t101615 / 18.0;
    (t102169, t102173, t102175, t102176, t102181)
}
