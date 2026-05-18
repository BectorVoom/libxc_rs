//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 793/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk793<F: Float>(t12797: F, t1358: F, t31591: F, t4261: F, t9074: F, t2321: F, t34600: F, t12830: F, t29874: F, t12803: F, t31586: F, t34604: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42673 = t1358 * t12797;
    let t42717 = t9074 * t4261 * t31591;
    let t42721 = t9074 * t34600 * t2321;
    let t42820 = t29874 * t12830;
    let t42825 = t1358 * t12803;
    let t42827 = t29874 * t12797;
    let t42846 = t29874 * t12803;
    let t42849 = t9074 * t4261 * t31586;
    let t42898 = t9074 * t34604 * t2321;
    (t42673, t42717, t42721, t42820, t42825, t42827, t42846, t42849, t42898)
}
