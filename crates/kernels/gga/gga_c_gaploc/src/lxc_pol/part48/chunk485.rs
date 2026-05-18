//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 485/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk485<F: Float>(t1359: F, t986: F, t107: F, t7887: F, t544: F, t2760: F, t1339: F, t2754: F, t1: F, t8025: F, t1415: F, t2967: F, t747: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8237 = t1359 * t986;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    let t8261 = t2760 * t107;
    let t8272 = t1339 * t2754;
    let t8330 = t8025 * t1;
    let t8331 = t544 * t8330;
    let t8410 = t7887 * t1;
    let t8411 = t1415 * t8410;
    let t8440 = t2967 * t747;
    (t8237, t8247, t8248, t8261, t8272, t8331, t8410, t8411, t8440)
}
