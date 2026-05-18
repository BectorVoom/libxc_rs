//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 872/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk872<F: Float>(t9524: F, t9542: F, t13346: F, t2320: F, t701: F, t3700: F, t9483: F, t173: F, t2440: F, t3691: F, t13309: F, t3806: F) -> (F, F, F, F, F, F) {
    let t13589 = t9524 * t9542;
    let t13592 = t2320 * t13346;
    let t13593 = t701 * t13592;
    let t13595 = t9483 * t3700;
    let t13596 = t701 * t13595;
    let t13598 = t173 * t2440;
    let t13599 = t13598 * t3691;
    let t13600 = t701 * t13599;
    let t13601 = F::new(0.56749874115226337448e-2) * t13600;
    let t13602 = t3806 * t13309;
    (t13589, t13593, t13596, t13600, t13601, t13602)
}
