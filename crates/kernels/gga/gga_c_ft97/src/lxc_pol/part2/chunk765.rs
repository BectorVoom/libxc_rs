//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 765/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk765<F: Float>(t13592: F, t701: F, t3700: F, t9483: F, t173: F, t2440: F, t3691: F, t13309: F, t3806: F, t420: F, t9651: F, t13315: F, t2248: F, t13320: F, t13296: F, t2320: F) -> (F, F, F, F, F, F, F, F) {
    let t13593 = t701 * t13592;
    let t13595 = t9483 * t3700;
    let t13596 = t701 * t13595;
    let t13598 = t173 * t2440;
    let t13599 = t13598 * t3691;
    let t13600 = t701 * t13599;
    let t13601 = 0.56749874115226337448e-2 * t13600;
    let t13602 = t3806 * t13309;
    let t13603 = t701 * t13602;
    let t13605 = t420 * t9651;
    let t13606 = t13605 * t13315;
    let t13607 = t701 * t13606;
    let t13609 = t2248 * t2440;
    let t13610 = t13609 * t13320;
    let t13611 = t701 * t13610;
    let t13613 = t2320 * t13296;
    (t13593, t13596, t13600, t13601, t13603, t13607, t13611, t13613)
}
