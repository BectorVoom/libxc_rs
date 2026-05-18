//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1133/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1133<F: Float>(t6255: F, t7561: F, t6260: F, t30543: F, t9720: F, t1797: F, t2020: F, t5586: F, t570: F, t2060: F, t6313: F, t7815: F) -> (F, F, F, F, F, F) {
    let t39605 = t7561 * t6255;
    let t39607 = t7561 * t6260;
    let t39609 = t30543 * t9720;
    let t39615 = t2020 * t1797;
    let t39617 = t570 * t5586;
    let t39620 = t2060 * t7815 * t6313;
    (t39605, t39607, t39609, t39615, t39617, t39620)
}
