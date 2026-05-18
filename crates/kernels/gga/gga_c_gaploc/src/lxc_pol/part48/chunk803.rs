//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 803/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk803<F: Float>(t12770: F, t484: F, t12830: F, t29874: F, t12803: F, t1358: F, t12797: F, t31586: F, t4261: F, t9074: F, t12820: F, t2312: F) -> (F, F, F, F, F, F, F) {
    let t42774 = t484 * t12770;
    let t42820 = t29874 * t12830;
    let t42825 = t1358 * t12803;
    let t42827 = t29874 * t12797;
    let t42846 = t29874 * t12803;
    let t42849 = t9074 * t4261 * t31586;
    let t42885 = t2312 * t12820;
    (t42774, t42820, t42825, t42827, t42846, t42849, t42885)
}
