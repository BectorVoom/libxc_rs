//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 940/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk940<F: Float>(t17388: F, t6010: F, t4291: F, t5747: F, t4294: F, t2066: F, t4278: F, t2033: F, t4121: F, t4257: F, t12530: F, t5913: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t17389 = t6010 * t17388;
    let t17391 = t5747 * t4291;
    let t17392 = t17391 * t4294;
    let t17394 = t4278 * t2066;
    let t17396 = t2033 * t4121;
    let t17397 = t17396 * sigma2;
    let t17398 = t17397 * t4257;
    let t17400 = t12530 * t5913;
    (t17389, t17392, t17394, t17396, t17398, t17400)
}
