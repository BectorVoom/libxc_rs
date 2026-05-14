//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 677/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk677<F: Float>(t2366: F, t36210: F, t1352: F, t3530: F, t3516: F, t447: F, t6508: F, t1959: F, t3634: F, t107: F, t11679: F, t1858: F, t3614: F, t11764: F, t783: F, t1: F, t35659: F, t787: F) -> (F, F, F, F, F, F, F, F) {
    let t36211 = t2366 * t36210;
    let t36247 = t3530 * t1352;
    let t36273 = t3516 * t447;
    let t36274 = t6508 * t36273;
    let t36313 = t3634 * t1959;
    let t36364 = t11679 * t107;
    let t36390 = t1858 * t3614;
    let t36477 = t11764 * t783;
    let t36506 = t787 * t35659 * t1;
    (t36211, t36247, t36274, t36313, t36364, t36390, t36477, t36506)
}
