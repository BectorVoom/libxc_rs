//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 907/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk907<F: Float>(t35180: F, t9562: F, t10256: F, t30830: F, t913: F, t2482: F, t3358: F, t9263: F, t12957: F, t31356: F, t35216: F, t9287: F) -> (F, F, F, F, F) {
    let t41666 = t35180 * t9562;
    let t41667 = F::cast_from(0.20854452471912748891e0_f64) * t41666;
    let t41669 = t30830 * t913 * t10256;
    let t41670 = F::cast_from(0.59584149919750711116e-1_f64) * t41669;
    let t41672 = t9263 * t3358 * t2482;
    let t41674 = t31356 * t12957;
    let t41675 = F::cast_from(0.76685851907841499353e0_f64) * t41674;
    let t41676 = t35216 * t9287;
    (t41667, t41670, t41672, t41675, t41676)
}
