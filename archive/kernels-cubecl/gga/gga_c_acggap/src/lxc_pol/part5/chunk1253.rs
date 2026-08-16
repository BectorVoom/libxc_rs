//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1253/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1253<F: Float>(t5821: F, t997: F, t5811: F, t5546: F, t14056: F, t6140: F, t3391: F, t4680: F, t6143: F, t1181: F, t1432: F, t15995: F) -> (F, F, F, F, F, F) {
    let t23063 = t997 * t5821;
    let t23065 = t997 * t5811;
    let t23068 = t997 * t5546;
    let t23070 = t14056 * t6140;
    let t23077 = t3391 * t4680 * t6143;
    let t23081 = t3391 * t1181 * t15995 * t1432;
    (t23063, t23065, t23068, t23070, t23077, t23081)
}
