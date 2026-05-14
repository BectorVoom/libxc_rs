//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 675/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk675<F: Float>(t3804: F, t3856: F, t3861: F, t3865: F, t1511: F, t793: F, t184: F, t199: F, t1519: F, t795: F, t2123: F, t565: F, t790: F, t925: F, t1968: F, t325: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4584 = 16.0 / 135.0 * t3804;
    let t4585 = 32.0 / 135.0 * t3856;
    let t4586 = 32.0 / 135.0 * t3861;
    let t4587 = 16.0 / 135.0 * t3865;
    let t4588 = t1511 * t793;
    let t4589 = t4588 * t184;
    let t4591 = 4.0 / 15.0 * t4589 * t199;
    let t4592 = t795 * t1519;
    let t4593 = 4.0 / 135.0 * t4592;
    let t4595 = 8.0 / 45.0 * t565 * t2123;
    let t4600 = t925 * t790;
    let t4602 = t325 * t1968;
    (t4584, t4585, t4586, t4587, t4588, t4589, t4591, t4593, t4595, t4600, t4602)
}
