//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 854/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk854<F: Float>(t40252: F, t40258: F, t40261: F, t40277: F, t40280: F, t40283: F, t1445: F, t1562: F, t41784: F, t12881: F, t9497: F, t8248: F, t9565: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41979 = F::new(0.29792074959875355558e-1) * t40252;
    let t41980 = F::new(0.20854452471912748891e0) * t40258;
    let t41981 = F::new(0.12780975317973583225e0) * t40261;
    let t41982 = F::new(0.17875244975925213335e0) * t40277;
    let t41983 = F::new(0.11916829983950142223e0) * t40280;
    let t41984 = F::new(0.59584149919750711116e-1) * t40283;
    let t41987 = F::new(0.62115540045351614476e2) * t1562 * t1445 * t41784;
    let t41989 = F::new(0.25025342966295298669e1) * t9497 * t12881;
    let t41991 = F::new(0.11916829983950142223e0) * t8248 * t9565;
    (t41979, t41980, t41981, t41982, t41983, t41984, t41987, t41989, t41991)
}
