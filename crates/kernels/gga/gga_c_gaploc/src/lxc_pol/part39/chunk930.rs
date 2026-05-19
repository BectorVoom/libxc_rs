//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 930/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk930<F: Float>(t40252: F, t40258: F, t40277: F, t40280: F, t40283: F, t1445: F, t1562: F, t41784: F, t12881: F, t9497: F, t8248: F, t9565: F) -> (F, F, F, F, F, F, F, F) {
    let t41979 = F::cast_from(0.29792074959875355558e-1_f64) * t40252;
    let t41980 = F::cast_from(0.20854452471912748891e0_f64) * t40258;
    let t41982 = F::cast_from(0.17875244975925213335e0_f64) * t40277;
    let t41983 = F::cast_from(0.11916829983950142223e0_f64) * t40280;
    let t41984 = F::cast_from(0.59584149919750711116e-1_f64) * t40283;
    let t41987 = F::cast_from(0.62115540045351614476e2_f64) * t1562 * t1445 * t41784;
    let t41989 = F::cast_from(0.25025342966295298669e1_f64) * t9497 * t12881;
    let t41991 = F::cast_from(0.11916829983950142223e0_f64) * t8248 * t9565;
    (t41979, t41980, t41982, t41983, t41984, t41987, t41989, t41991)
}
