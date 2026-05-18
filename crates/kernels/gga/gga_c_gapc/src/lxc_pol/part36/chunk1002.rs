//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1002/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1002<F: Float>(t11659: F, t11664: F, t11666: F, t11671: F, t11676: F, t11680: F, t11685: F, t11689: F, t11692: F, t11696: F, t11699: F, t11701: F, t11703: F) -> F {
    let t12463 = -F::new(0.97858176633505899139e-7) * t11659 + F::new(0.12843885683147649262e-5) * t11664 - F::new(0.46971924784082831588e-4) * t11666 - F::new(0.46971924784082831588e-4) * t11671 + F::new(0.68394856556563412154e-6) * t11676 + F::new(0.68394856556563412154e-6) * t11680 - F::new(0.29357452990051769742e-5) * t11685 - F::new(0.29357452990051769742e-5) * t11689 - F::new(0.83516082266099274563e-5) * t11692 + F::new(0.22798285518854470718e-6) * t11696 - F::new(0.10943177049050145945e-4) * t11699 + F::new(0.32829531147150437834e-4) * t11701 - F::new(0.14226130163765189728e-3) * t11703;
    t12463
}
