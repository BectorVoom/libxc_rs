//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk754<F: Float>(t4562: F, t4565: F, t4572: F, t2425: F, t835: F, t6597: F, t786: F, t6601: F, t813: F, t2473: F, t795: F, t4592: F, t4185: F, t4198: F, t4201: F, t4206: F, t4209: F, t4544: F, t4547: F, t4719: F, t7256: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7530 = 4.0 / 45.0 * t4562;
    let t7531 = 8.0 / 45.0 * t4565;
    let t7532 = 8.0 / 45.0 * t4572;
    let t7534 = 2.0 / 5.0 * t2425 * t835;
    let t7536 = 4.0 / 5.0 * t6597 * t786;
    let t7538 = 4.0 / 5.0 * t6601 * t813;
    let t7540 = 4.0 / 5.0 * t795 * t2473;
    let t7541 = 4.0 / 45.0 * t4592;
    let t7544 = 0.21642082724729686 * t4544 + 0.03354522822333102 * t4547 - t4185 + t4198 + t4201 + t4206 - t4209 + t7530 - t7531 - t7532 - t7534 + t7536 + t7538 + t7540 - t7541 + 4.0 * t4719 + 4.0 * t7256;
    (t7530, t7531, t7532, t7534, t7536, t7538, t7540, t7541, t7544)
}
