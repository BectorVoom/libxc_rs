//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1104/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1104<F: Float>(t158: F, t27335: F, t526: F, t9439: F, t27326: F, t8392: F, t160: F, t26768: F, t605: F, t9016: F, t27208: F, t27212: F, t27022: F, t6649: F, t8232: F, t23571: F, t50249: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t106651 = t158 * t27335;
    let t106698 = t526 * t9439;
    let t106708 = 2.0 / 27.0 * t8392 * t27326;
    let t106724 = t160 * t26768;
    let t106729 = t9016 * t605;
    let t106745 = 2.0 / 27.0 * t8392 * t27208;
    let t106747 = 4.0 / 27.0 * t8392 * t27212;
    let t106759 = 2.0 / 27.0 * t8392 * t27022;
    let t106798 = t8232 * t6649;
    let t106803 = t50249 * t23571;
    (t106651, t106698, t106708, t106724, t106729, t106745, t106747, t106759, t106798, t106803)
}
