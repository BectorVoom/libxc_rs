//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 842/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk842<F: Float>(t2415: F, t833: F, t1308: F, t571: F, t2396: F, t4479: F, t3965: F, t2388: F, t4475: F, t3974: F, t6791: F, t2499: F, t795: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7745 = t2415 * t833;
    let t7746 = t1308 * t7745;
    let t7748 = F::new(8.0) / F::new(15.0) * t571 * t7746;
    let t7749 = t4479 * t2396;
    let t7751 = F::new(16.0) / F::new(15.0) * t3965 * t7749;
    let t7752 = t4475 * t2388;
    let t7754 = F::new(16.0) / F::new(15.0) * t3974 * t7752;
    let t7755 = F::new(8.0) / F::new(15.0) * t6791;
    let t7757 = F::new(2.0) / F::new(5.0) * t795 * t2499;
    (t7745, t7746, t7748, t7749, t7751, t7752, t7754, t7755, t7757)
}
