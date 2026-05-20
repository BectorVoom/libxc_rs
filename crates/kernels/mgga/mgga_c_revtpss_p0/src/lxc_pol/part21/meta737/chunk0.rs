//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2588/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2588<F: Float>(t123: F, t2434: F, t4077: F, t9680: F, t125: F, t1358: F, t555: F, t8779: F, t9645: F, t1445: F, t689: F, t9634: F) -> (F, F, F) {
    let t47580 = t9680 * t123 * t2434 * t4077;
    let t47591 = F::cast_from(0.65457331274007190912e-5_f64) * t123 * t125 * t8779 * t9645 * t555 * t1358;
    let t47593 = t689 * t9634 * t1445;
    (t47580, t47591, t47593)
}
