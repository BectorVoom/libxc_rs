//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 939/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk939<F: Float>(t38111: F, t528: F, t118: F, t37993: F, t37640: F, t38062: F, t38046: F, t38050: F, t38069: F, t38077: F, t38081: F, t38084: F, t38088: F, t38090: F, t38101: F, t38108: F, t39533: F) -> (F, F, F) {
    let t39535 = t528 * t38111;
    let t39538 = F::new(1.0) / t118 / t37993;
    let t39539 = t39538 * t37640;
    let t39546 = F::new(0.14978012345679012345e1) * t38062;
    let t39550 = -F::new(0.51995099999999999998e1) * t38101 + F::new(0.11554466666666666666e1) * t38108 - F::new(0.352131e0) * t39533 + F::new(0.234754e0) * t39535 - F::new(0.44016375e0) * t39539 + F::new(0.38514888888888888888e0) * t38046 - F::new(0.11554466666666666666e1) * t38050 - F::new(0.9628722222222222222e0) * t38069 + F::new(0.34663399999999999999e1) * t38077 - F::new(0.38514888888888888888e0) * t38084 + t39546 - F::new(0.28886166666666666666e0) * t38081 + F::new(0.34663399999999999999e1) * t38088 + F::new(0.59912049382716049381e0) * t38090;
    (t39535, t39539, t39550)
}
