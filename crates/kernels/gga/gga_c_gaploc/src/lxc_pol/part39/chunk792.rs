//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 792/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk792<F: Float>(t12957: F, t31356: F, t35216: F, t9287: F, t2875: F, t4386: F, t544: F, t9078: F, t2792: F, t3177: F, t9263: F, t9278: F, t993: F, t20671: F, t31041: F, t34818: F) -> (F, F, F, F, F, F) {
    let t41674 = t31356 * t12957;
    let t41675 = 0.76685851907841499353e0 * t41674;
    let t41676 = t35216 * t9287;
    let t41677 = 0.29792074959875355558e-1 * t41676;
    let t41681 = 0.27805936629216998521e0 * t544 * t9078 * t2875 * t4386;
    let t41683 = t9263 * t2792 * t3177;
    let t41684 = 0.76685851907841499353e0 * t41683;
    let t41686 = t9263 * t993 * t9278;
    let t41687 = 0.76685851907841499353e0 * t41686;
    let t41689 = t31041 * t20671 * t34818;
    (t41675, t41677, t41681, t41684, t41687, t41689)
}
