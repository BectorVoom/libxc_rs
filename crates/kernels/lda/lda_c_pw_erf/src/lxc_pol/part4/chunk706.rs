//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 706/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk706<F: Float>(t173: F, t4645: F, t184: F, t199: F, t4206: F, t4209: F, t4563: F, t4566: F, t4570: F, t4573: F, t4578: F, t4580: F, t4583: F, t4584: F, t4585: F, t4586: F, t4587: F, t4591: F, t4593: F, t4595: F) -> (F, F, F, F) {
    let t4646 = t173 * t4645;
    let t4647 = t4646 * t184;
    let t4649 = 2.0 / 15.0 * t4647 * t199;
    let t4650 = t4206 - t4209 + t4563 - t4566 - t4570 - t4573 - t4578 - t4580 + t4583 - t4584 + t4585 + t4586 - t4587 + t4591 - t4593 - t4595 + t4649;
    (t4646, t4647, t4649, t4650)
}
