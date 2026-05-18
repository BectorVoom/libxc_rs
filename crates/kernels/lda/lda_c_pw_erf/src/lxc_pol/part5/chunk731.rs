//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 731/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk731<F: Float>(t186: F, t6591: F, t493: F, t6475: F, t6477: F, t6481: F, t6485: F, t6487: F, t6491: F, t6495: F, t6570: F, t6572: F, t6574: F, t6576: F, t6578: F, t6582: F, t6584: F, t6586: F, t6588: F) -> (F, F, F) {
    let t6592 = t186 * t6591;
    let t6594 = F::new(4.0) / F::new(15.0) * t493 * t6592;
    let t6595 = t6475 + t6477 - t6481 - t6485 - t6487 + t6491 + t6495 - t6570 + t6572 - t6574 - t6576 - t6578 + t6582 + t6584 + t6586 + t6588 + t6594;
    (t6592, t6594, t6595)
}
