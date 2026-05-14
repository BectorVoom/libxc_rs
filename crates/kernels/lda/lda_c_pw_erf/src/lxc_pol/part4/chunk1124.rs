//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1124/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1124<F: Float>(t16304: F, t16343: F, t16358: F, t16391: F, t16424: F, t16443: F, t16484: F, t16506: F, t186: F, t211: F, t582: F, t2140: F, t5334: F, t2146: F, t4901: F, t1341: F, t6198: F) -> (F, F, F, F) {
    let t16513 = 2.0 / 15.0 * t211 * t186 * t582 * (t16304 + t16343 + t16358 + t16391 + t16424 + t16443 + t16484 + t16506);
    let t16514 = t5334 * t2140;
    let t16515 = 32.0 / 135.0 * t16514;
    let t16516 = t2146 * t4901;
    let t16517 = 16.0 / 27.0 * t16516;
    let t16519 = 8.0 / 45.0 * t6198 * t1341;
    (t16513, t16515, t16517, t16519)
}
