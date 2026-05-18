//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 886/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk886<F: Float>(t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t16743: F, t16747: F, t16750: F, t16756: F, t16759: F, t16763: F, t16766: F) -> F {
    let t16769 = F::new(0.51647499999999999999e0) * t13703 + F::new(0.3529725e1) * t16743 - F::new(0.516475e0) * t16650 - F::new(0.62517e0) * t16747 + F::new(0.20839e0) * t16750 + F::new(0.20659e1) * t16634 - F::new(0.309885e1) * t16642 - F::new(0.57386111111111111112e0) * t16630 - F::new(0.46308888888888888889e-1) * t16756 - F::new(0.104195e0) * t16759 + F::new(0.309885e1) * t16646 + F::new(0.62517e0) * t16763 - F::new(0.104195e0) * t16766 - F::new(0.103295e1) * t16638;
    t16769
}
