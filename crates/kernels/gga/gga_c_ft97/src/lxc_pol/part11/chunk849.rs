//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 849/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk849<F: Float>(t643: F, t8376: F, t12168: F, t70: F, t170: F, t180: F, t2253: F, t8711: F, t3628: F, t645: F, t2294: F, t8621: F, t178: F, t2280: F, t2282: F, t2296: F, t8640: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39575 = t8376 * t643;
    let t39600 = t12168 * t70;
    let t39603 = 220.0 / 81.0 * t170 * t39600 * t180;
    let t39604 = t2253 * t8711;
    let t39606 = t3628 * t645;
    let t39608 = t2294 * t2294;
    let t39613 = t2253 * t8621;
    let t39616 = 1.0 / t2280 / t178;
    let t39617 = t2282 * t2282;
    let t39622 = t8640 * t2296;
    (t39575, t39600, t39603, t39604, t39606, t39608, t39613, t39616, t39617, t39622)
}
