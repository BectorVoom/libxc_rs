//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2372/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2372<F: Float>(t138: F, t785: F, t9302: F, t2786: F, t234: F, t39545: F, t685: F, t875: F, t2778: F, t39515: F, t39501: F, t871: F) -> (F, F, F, F, F) {
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40294 = F::cast_from(0.65457331274007190912e-5_f64) * t39545 * t234 * t875 * t685;
    let t40314 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t2778;
    let t40316 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t871;
    (t40270, t40271, t40294, t40314, t40316)
}
