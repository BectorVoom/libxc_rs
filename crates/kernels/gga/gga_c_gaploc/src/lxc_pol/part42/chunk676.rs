//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 676/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk676<F: Float>(t11218: F, t203: F, t599: F, t3529: F, t447: F, t2366: F, t1352: F, t3530: F, t3516: F, t6508: F, t1959: F, t3634: F, t107: F, t11679: F, t1858: F, t3614: F) -> (F, F, F, F, F, F, F, F) {
    let t36117 = t203 * t11218;
    let t36178 = t599 * t11218;
    let t36210 = t3529 * t447;
    let t36211 = t2366 * t36210;
    let t36247 = t3530 * t1352;
    let t36273 = t3516 * t447;
    let t36274 = t6508 * t36273;
    let t36313 = t3634 * t1959;
    let t36364 = t11679 * t107;
    let t36390 = t1858 * t3614;
    (t36117, t36178, t36211, t36247, t36274, t36313, t36364, t36390)
}
