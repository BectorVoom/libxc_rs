//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 686/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk686<F: Float>(t11360: F, t371: F, t408: F, t929: F, t11174: F, t17: F, t355: F, t3001: F, t89: F, t3014: F, t376: F, t3196: F, t8392: F) -> (F, F, F, F, F, F, F, F) {
    let t11361 = t371 * t11360;
    let t11375 = t408 * t929;
    let t11401 = t11174 * t17;
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    let t11416 = t89 * t376 * t3014;
    let t11417 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11416;
    let t11430 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8392 * t3196;
    (t11361, t11375, t11401, t11402, t11404, t11416, t11417, t11430)
}
