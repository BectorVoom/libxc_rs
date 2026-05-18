//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 921/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk921<F: Float>(t2325: F, t31501: F, t882: F, t883: F, t2268: F, t3158: F, t8195: F, t8199: F, t9181: F, t2321: F, t34604: F, t9074: F) -> (F, F, F, F) {
    let t42889 = t882 * t2325 * t883 * t31501;
    let t42893 = F::new(0.42682509953514224398e0) * t2268 * t3158 * t8195;
    let t42896 = F::new(0.14227503317838074799e1) * t2268 * t9181 * t8199;
    let t42898 = t9074 * t34604 * t2321;
    (t42889, t42893, t42896, t42898)
}
