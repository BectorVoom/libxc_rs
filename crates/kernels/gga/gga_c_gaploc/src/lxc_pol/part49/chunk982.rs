//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 982/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk982<F: Float>(t1063: F, t32067: F, t894: F, t12820: F, t2312: F, t2325: F, t31501: F, t882: F, t883: F, t2268: F, t3158: F, t8195: F) -> (F, F, F, F) {
    let t42883 = t1063 * t894 * t32067;
    let t42885 = t2312 * t12820;
    let t42889 = t882 * t2325 * t883 * t31501;
    let t42893 = F::new(0.42682509953514224398e0) * t2268 * t3158 * t8195;
    (t42883, t42885, t42889, t42893)
}
