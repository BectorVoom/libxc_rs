//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 893/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk893<F: Float>(t1165: F, t22040: F, t7351: F, t7493: F, t1181: F, t20311: F, t7426: F, t21118: F, t8600: F, t7637: F, t8555: F, t1967: F, t8549: F, t30219: F, t8515: F, t1345: F, t1992: F) -> (F, F, F, F, F, F, F) {
    let t35190 = t7493 * t1165 * t7351 * t22040;
    let t35194 = t7426 * t1181 * t7351 * t20311;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35204 = t7637 * t8555;
    let t35210 = t1967 * t8549;
    let t35212 = t30219 * t8515;
    let t35225 = t1992 * t1345;
    (t35190, t35194, t35198, t35204, t35210, t35212, t35225)
}
