//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 526/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk526<F: Float>(t1887: F, t1901: F, t28: F, t3177: F, t3224: F, t3260: F, t3286: F, t446: F, t4547: F, t4553: F, t4557: F, t4561: F, t4565: F, t4569: F, t4574: F, t4591: F, t4595: F, t4599: F, t4603: F, t4608: F, t4613: F, t4617: F, t89: F) -> (F,) {
    let t4621 = t89 * t28 * t4547 / 3.0 + 2.0 / 3.0 * t446 * t4553 - 2.0 / 9.0 * t446 * t4557 - t446 * t4561 / 9.0 - 2.0 / 27.0 * t446 * t4565 + 2.0 / 3.0 * t446 * t4569 + 2.0 / 3.0 * t446 * t4574 + 2.0 / 9.0 * t3224 + 2.0 / 9.0 * t3260 + t1887 - 2.0 / 9.0 * t3177 - t446 * t4591 / 3.0 - 2.0 / 3.0 * t446 * t4595 - 2.0 / 3.0 * t446 * t4599 - t446 * t4603 / 3.0 + 2.0 / 9.0 * t1901 * t4608 + 2.0 / 9.0 * t1901 * t4613 + 2.0 / 9.0 * t446 * t4617 + 2.0 / 27.0 * t3286;
    (t4621,)
}
