//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 714/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk714<F: Float>(t3700: F, t9483: F, t701: F, t173: F, t2440: F, t3691: F, t420: F, t9651: F, t2248: F, t703: F, t3813: F, t8715: F) -> (F, F, F, F, F, F, F) {
    let t13595 = t9483 * t3700;
    let t13596 = t701 * t13595;
    let t13598 = t173 * t2440;
    let t13599 = t13598 * t3691;
    let t13600 = t701 * t13599;
    let t13601 = F::cast_from(0.56749874115226337448e-2_f64) * t13600;
    let t13605 = t420 * t9651;
    let t13609 = t2248 * t2440;
    let t13616 = t2248 * t703;
    let t13628 = t8715 * t3813;
    (t13596, t13600, t13601, t13605, t13609, t13616, t13628)
}
