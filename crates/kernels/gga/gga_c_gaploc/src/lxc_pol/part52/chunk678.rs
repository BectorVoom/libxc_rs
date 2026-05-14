//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 678/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk678<F: Float>(t107: F, t35439: F, t787: F, t11613: F, t769: F, t11822: F, t1980: F, t36364: F, t1858: F, t3601: F, t6058: F, t11595: F, t11576: F, t795: F, t313: F, t1: F, t36610: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36700 = t787 * t35439 * t107;
    let t36738 = t769 * t11613;
    let t36762 = t1980 * t11822;
    let t36782 = t787 * t36364;
    let t36798 = t1858 * t3601;
    let t37032 = t6058 * t3601;
    let t37057 = t769 * t11595;
    let t37060 = t795 * t11576;
    let t37061 = t313 * t37060;
    let t37179 = t36610 * t1;
    (t36700, t36738, t36762, t36782, t36798, t37032, t37057, t37060, t37061, t37179)
}
