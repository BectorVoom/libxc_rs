//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 985/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk985<F: Float>(t142: F, t2030: F, t5506: F, t599: F, t2060: F, t361: F, t9704: F, t1165: F, t5969: F, t604: F, t7493: F, t1992: F, t30692: F, t7842: F, t9587: F, t7839: F, t9601: F) -> (F, F, F, F, F) {
    let t39334 = t2030 * t142 * t599 * t5506;
    let t39337 = t2060 * t361 * t9704;
    let t39343 = t7493 * t1165 * t604 * t5969;
    let t39356 = t30692 * t7842 * t1992 * t9587;
    let t39358 = t7839 * t9601;
    (t39334, t39337, t39343, t39356, t39358)
}
