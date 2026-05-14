//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1181/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1181<F: Float>(t25877: F, t94390: F, t94385: F, t9675: F, t7289: F, t94377: F, t122: F, t72: F, t7274: F, t3916: F, t25895: F, t7285: F, t9288: F, t7284: F, t25884: F, t686: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94589 = t94390 * t25877;
    let t94590 = t94385 * t9675;
    let t94591 = t94589 * t94590;
    let t94593 = t7289 * t94377;
    let t94596 = t7274 * t72 * t122;
    let t94597 = t94596 * t3916;
    let t94598 = t25895 * t94597;
    let t94600 = t7285 * t9288;
    let t94602 = 0.22487184191643109717e-1 * t7284 * t94600;
    let t94604 = t25884 * t72 * t686;
    (t94590, t94591, t94593, t94596, t94597, t94598, t94600, t94602, t94604)
}
