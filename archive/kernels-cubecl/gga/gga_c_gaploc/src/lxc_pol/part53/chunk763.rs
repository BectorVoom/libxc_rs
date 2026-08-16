//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 763/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk763<F: Float>(t2366: F, t30208: F, t1359: F, t3116: F, t3085: F, t1: F, t29882: F, t544: F, t1397: F, t9290: F, t2321: F, t28438: F) -> (F, F, F, F, F, F) {
    let t30209 = t2366 * t30208;
    let t30301 = t1359 * t3116;
    let t30334 = t1359 * t3085;
    let t30635 = t544 * t29882 * t1;
    let t30639 = t1397 * t9290;
    let t30733 = t28438 * t2321;
    (t30209, t30301, t30334, t30635, t30639, t30733)
}
