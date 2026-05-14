//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 957/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk957<F: Float>(t1298: F, t560: F, t469: F, t5506: F, t157: F, t1914: F, t406: F, t1814: F, t33795: F, t615: F, t1839: F, t463: F, t5873: F, t7822: F, t7493: F, t8480: F, t8648: F) -> (F, F, F, F, F, F, F, F) {
    let t38563 = t1298 * t560;
    let t38573 = t469 * t5506;
    let t38635 = t1914 * t406 * t157;
    let t38647 = t1814 * t406 * t157;
    let t38662 = t615 * t33795;
    let t38685 = t1839 * t463;
    let t38701 = t7822 * t5873;
    let t38704 = t7493 * t8480 * t8648;
    (t38563, t38573, t38635, t38647, t38662, t38685, t38701, t38704)
}
