//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 675/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk675<F: Float>(t29975: F, t6508: F, t2293: F, t874: F, t172: F, t20368: F, t2366: F, t29853: F, t4260: F, t883: F, t3116: F, t447: F, t1359: F, t3085: F, t1: F, t29882: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29976 = t6508 * t29975;
    let t29984 = t874 * t2293;
    let t29985 = t6508 * t29984;
    let t30019 = t172 * t2293;
    let t30136 = t20368 * t29975;
    let t30140 = t2366 * t29853;
    let t30204 = t4260 * t883;
    let t30208 = t3116 * t447;
    let t30209 = t2366 * t30208;
    let t30301 = t1359 * t3116;
    let t30334 = t1359 * t3085;
    let t30635 = t544 * t29882 * t1;
    (t29976, t29984, t29985, t30019, t30136, t30140, t30204, t30208, t30209, t30301, t30334, t30635)
}
