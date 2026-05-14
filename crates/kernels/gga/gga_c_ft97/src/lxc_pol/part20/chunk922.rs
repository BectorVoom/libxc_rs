//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 922/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk922<F: Float>(t28628: F, t811: F, t25057: F, t820: F, t1196: F, t218: F, t1472: F, t27720: F, t2691: F, t7005: F, t213: F, t231: F, t6819: F, t2724: F, t39: F, t5585: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28629 = t28628 * t811;
    let t28630 = t25057 * t28629;
    let t28633 = t28628 * t820;
    let t28634 = t25057 * t28633;
    let t28637 = t218 * t1196;
    let t28638 = t28637 * t820;
    let t28639 = t25057 * t28638;
    let t28646 = t1472 * t27720;
    let t28652 = t2691 * t7005;
    let t28654 = t231 * t213 * t811;
    let t28655 = t6819 * t28654;
    let t28658 = t2724 * t39;
    let t28659 = t28658 * t5585;
    (t28630, t28634, t28637, t28639, t28646, t28652, t28654, t28655, t28658, t28659)
}
