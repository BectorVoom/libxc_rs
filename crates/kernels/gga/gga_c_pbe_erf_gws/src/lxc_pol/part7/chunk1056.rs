//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1056/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1056<F: Float>(t2323: F, t6451: F, t4395: F, t6638: F, t4408: F, t6670: F, t822: F, t6680: F, t4413: F, t6673: F, t6672: F, t6501: F, t6548: F, t1452: F, t814: F, t2306: F, t3074: F, t339: F, t6104: F, t860: F) -> (F, F, F, F, F, F, F) {
    let t21146 = t2323 * t6451;
    let t21148 = t4395 * t6638;
    let t21152 = t4408 * t6670;
    let t21153 = t822 * t21152;
    let t21155 = t21153 * t6680 / 12.0;
    let t21156 = t4413 * t6673;
    let t21157 = t6672 * t21156;
    let t21158 = 7.0 / 6.0 * t21157;
    let t21159 = t6501 * t6548;
    let t21161 = t1452 * t814;
    let t21174 = t3074 * t2306 * t6104 * t339 * t860 / 24.0;
    (t21146, t21148, t21155, t21158, t21159, t21161, t21174)
}
