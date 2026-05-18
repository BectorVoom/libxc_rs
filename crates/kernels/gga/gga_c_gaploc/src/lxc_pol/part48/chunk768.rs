//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 768/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk768<F: Float>(t36364: F, t787: F, t1858: F, t3601: F, t6058: F, t11595: F, t769: F, t11576: F, t795: F, t313: F, t8748: F, t1: F, t36610: F) -> (F, F, F, F, F, F, F, F) {
    let t36782 = t787 * t36364;
    let t36798 = t1858 * t3601;
    let t37032 = t6058 * t3601;
    let t37057 = t769 * t11595;
    let t37060 = t795 * t11576;
    let t37061 = t313 * t37060;
    let t37166 = t787 * t8748;
    let t37179 = t36610 * t1;
    (t36782, t36798, t37032, t37057, t37060, t37061, t37166, t37179)
}
