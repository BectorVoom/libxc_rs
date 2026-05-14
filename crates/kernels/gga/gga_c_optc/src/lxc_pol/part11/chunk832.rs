//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 832/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk832<F: Float>(t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t16743: F, t16747: F, t16750: F, t16756: F, t16759: F, t16763: F, t16766: F, t16741: F, t818: F) -> (F, F) {
    let t16769 = 0.51647499999999999999e0 * t13703 + 0.3529725e1 * t16743 - 0.516475e0 * t16650 - 0.62517e0 * t16747 + 0.20839e0 * t16750 + 0.20659e1 * t16634 - 0.309885e1 * t16642 - 0.57386111111111111112e0 * t16630 - 0.46308888888888888889e-1 * t16756 - 0.104195e0 * t16759 + 0.309885e1 * t16646 + 0.62517e0 * t16763 - 0.104195e0 * t16766 - 0.103295e1 * t16638;
    let t16770 = t16741 + t16769;
    let t16771 = t16770 * t818;
    (t16770, t16771)
}
