//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 940/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk940<F: Float>(t213: F, t231: F, t811: F, t6819: F, t2724: F, t39: F, t5585: F, t4113: F) -> (F, F, F, F, F) {
    let t28654 = t231 * t213 * t811;
    let t28655 = t6819 * t28654;
    let t28658 = t2724 * t39;
    let t28659 = t28658 * t5585;
    let t28660 = t4113 * t28659;
    (t28654, t28655, t28658, t28659, t28660)
}
