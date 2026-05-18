//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1012/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1012<F: Float>(t30374: F, t8606: F, t7426: F, t7569: F, t8480: F, t7433: F, t8481: F, t34161: F, t8465: F, t31421: F, t1992: F, t7585: F, t7842: F, t8402: F) -> (F, F, F, F, F, F) {
    let t35587 = t30374 * t8606;
    let t35594 = t7426 * t8480 * t7569;
    let t35596 = t7433 * t8481;
    let t35601 = t34161 * t8465;
    let t35603 = F::new(0.22921875e-1) * t31421;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    (t35587, t35594, t35596, t35601, t35603, t35608)
}
