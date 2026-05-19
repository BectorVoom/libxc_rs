//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 802/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk802<F: Float>(t41060: F, t41071: F, t10040: F, t25198: F, t13055: F, t5640: F, t13058: F, t1991: F, t20671: F, t28309: F, t33601: F, t33565: F, t7372: F) -> (F, F, F, F, F, F, F) {
    let t43602 = F::cast_from(0.25561950635947166451e0_f64) * t41060;
    let t43604 = F::cast_from(0.25561950635947166451e0_f64) * t41071;
    let t43646 = t25198 * t10040;
    let t43652 = t5640 * t13055;
    let t43657 = t1991 * t13058;
    let t43660 = t28309 * t20671 * t33601;
    let t43679 = t33565 * t7372;
    (t43602, t43604, t43646, t43652, t43657, t43660, t43679)
}
