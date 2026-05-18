//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 821/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk821<F: Float>(t4562: F, t4565: F, t4572: F, t2425: F, t835: F, t6597: F, t786: F, t6601: F, t813: F, t2473: F, t795: F, t4592: F) -> (F, F, F, F, F, F, F, F) {
    let t7530 = F::new(4.0) / F::new(45.0) * t4562;
    let t7531 = F::new(8.0) / F::new(45.0) * t4565;
    let t7532 = F::new(8.0) / F::new(45.0) * t4572;
    let t7534 = F::new(2.0) / F::new(5.0) * t2425 * t835;
    let t7536 = F::new(4.0) / F::new(5.0) * t6597 * t786;
    let t7538 = F::new(4.0) / F::new(5.0) * t6601 * t813;
    let t7540 = F::new(4.0) / F::new(5.0) * t795 * t2473;
    let t7541 = F::new(4.0) / F::new(45.0) * t4592;
    (t7530, t7531, t7532, t7534, t7536, t7538, t7540, t7541)
}
