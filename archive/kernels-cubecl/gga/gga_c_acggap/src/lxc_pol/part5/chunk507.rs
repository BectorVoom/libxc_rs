//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 507/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk507<F: Float>(t2631: F, t680: F, t686: F, t286: F, t244: F, t712: F, t811: F, t814: F, t229: F, t804: F, t243: F, t803: F) -> (F, F, F, F, F, F, F, F) {
    let t2632 = t686 * t680 * t2631;
    let t2633 = t286 * t2632;
    let t2634 = F::cast_from(0.51947577317044391277e2_f64) * t2633;
    let t2635 = t712 * t244;
    let t2637 = t811 * t814;
    let t2641 = t229 * t804;
    let t2642 = F::cast_from(12.0_f64) * t2641;
    let t2643 = t243 * t803;
    (t2632, t2633, t2634, t2635, t2637, t2641, t2642, t2643)
}
