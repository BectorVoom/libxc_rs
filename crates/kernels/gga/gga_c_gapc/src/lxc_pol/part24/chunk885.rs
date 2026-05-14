//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 885/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk885<F: Float>(t11659: F, t11664: F, t11666: F, t11671: F, t11676: F, t11680: F, t11685: F, t11689: F, t11692: F, t11696: F, t11699: F, t11701: F, t11703: F, t12449: F, t3879: F, t883: F) -> (F, F) {
    let t12463 = -0.97858176633505899139e-7 * t11659 + 0.12843885683147649262e-5 * t11664 - 0.46971924784082831588e-4 * t11666 - 0.46971924784082831588e-4 * t11671 + 0.68394856556563412154e-6 * t11676 + 0.68394856556563412154e-6 * t11680 - 0.29357452990051769742e-5 * t11685 - 0.29357452990051769742e-5 * t11689 - 0.83516082266099274563e-5 * t11692 + 0.22798285518854470718e-6 * t11696 - 0.10943177049050145945e-4 * t11699 + 0.32829531147150437834e-4 * t11701 - 0.14226130163765189728e-3 * t11703;
    let t12464 = t12449 + t12463;
    let t12466 = t3879 * t883;
    (t12464, t12466)
}
