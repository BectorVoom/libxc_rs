//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 713/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk713<F: Float>(t14275: F, t14288: F, t209: F, t1016: F, t12032: F, t2798: F, t3718: F, t1382: F, t2854: F, t3689: F, t1445: F, t2778: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14289 = t14275 + t14288;
    let t14290 = t14289 * t209;
    let t14292 = F::new(2.0) * t12032 * t1016;
    let t14294 = F::new(2.0) * t2798 * t3718;
    let t14295 = t1016 * t3718;
    let t14297 = F::new(4.0) * t1382 * t14295;
    let t14298 = t2854 * t3689;
    let t14299 = t1445 * t14298;
    let t14302 = t2778 * t3689;
    (t14289, t14290, t14292, t14294, t14295, t14297, t14298, t14299, t14302)
}
