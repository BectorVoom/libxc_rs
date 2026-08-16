//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1005/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1005<F: Float>(t11659: F, t11664: F, t11666: F, t11671: F, t11676: F, t11680: F, t11685: F, t11689: F, t11692: F, t11696: F, t11699: F, t11701: F, t11703: F) -> F {
    let t12463 = -F::cast_from(0.97858176633505899139e-7_f64) * t11659 + F::cast_from(0.12843885683147649262e-5_f64) * t11664 - F::cast_from(0.46971924784082831588e-4_f64) * t11666 - F::cast_from(0.46971924784082831588e-4_f64) * t11671 + F::cast_from(0.68394856556563412154e-6_f64) * t11676 + F::cast_from(0.68394856556563412154e-6_f64) * t11680 - F::cast_from(0.29357452990051769742e-5_f64) * t11685 - F::cast_from(0.29357452990051769742e-5_f64) * t11689 - F::cast_from(0.83516082266099274563e-5_f64) * t11692 + F::cast_from(0.22798285518854470718e-6_f64) * t11696 - F::cast_from(0.10943177049050145945e-4_f64) * t11699 + F::cast_from(0.32829531147150437834e-4_f64) * t11701 - F::cast_from(0.14226130163765189728e-3_f64) * t11703;
    t12463
}
