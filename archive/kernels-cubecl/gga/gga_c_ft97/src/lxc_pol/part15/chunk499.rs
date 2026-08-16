//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 499/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk499<F: Float>(t2427: F, t5025: F, t14: F, t4995: F, t228: F, t231: F, t1124: F, t3799: F, t2441: F, t4917: F, t420: F, t701: F) -> (F, F, F, F, F, F) {
    let t5026 = t2427 * t5025;
    let t5029 = t4995 * t14;
    let t5031 = t228 * t5029 * t231;
    let t5034 = t3799 * t1124;
    let t5037 = t2441 * t4917;
    let t5038 = t420 * t5037;
    let t5039 = t701 * t5038;
    (t5026, t5031, t5034, t5037, t5038, t5039)
}
