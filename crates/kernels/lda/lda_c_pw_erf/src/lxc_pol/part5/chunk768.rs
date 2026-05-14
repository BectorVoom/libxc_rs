//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 768/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk768<F: Float>(t2396: F, t4479: F, t3965: F, t2388: F, t4475: F, t3974: F, t6791: F, t2499: F, t795: F, t1268: F, t7639: F, t7643: F, t538: F, t7647: F, t7651: F, t7655: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7749 = t4479 * t2396;
    let t7751 = 16.0 / 15.0 * t3965 * t7749;
    let t7752 = t4475 * t2388;
    let t7754 = 16.0 / 15.0 * t3974 * t7752;
    let t7755 = 8.0 / 15.0 * t6791;
    let t7757 = 2.0 / 5.0 * t795 * t2499;
    let t7758 = t1268 * t7639;
    let t7761 = t1268 * t7643;
    let t7764 = t538 * t7647;
    let t7767 = t538 * t7651;
    let t7770 = t538 * t7655;
    (t7749, t7751, t7752, t7754, t7755, t7757, t7758, t7761, t7764, t7767, t7770)
}
