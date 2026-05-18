//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1196/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1196<F: Float>(t17579: F, t17591: F, t17594: F, t2163: F, t7007: F, t15926: F, t6958: F, t518: F, t7469: F, t577: F, t7465: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t21694 = F::new(16.0) / F::new(15.0) * t17579;
    let t21695 = F::new(16.0) / F::new(15.0) * t17591;
    let t21696 = F::new(8.0) / F::new(5.0) * t17594;
    let t21698 = F::new(8.0) / F::new(5.0) * t7007 * t2163;
    let t21700 = F::new(8.0) / F::new(5.0) * t15926 * t6958;
    let t21701 = t7469 * t518;
    let t21703 = F::new(8.0) / F::new(15.0) * t21701 * t577;
    let t21704 = t7465 * t518;
    let t21706 = F::new(8.0) / F::new(15.0) * t21704 * t525;
    (t21694, t21695, t21696, t21698, t21700, t21703, t21706)
}
