//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1302/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1302<F: Float>(t20816: F, t20819: F, t20826: F, t20829: F, t20832: F, t20835: F, t20837: F, t20840: F, t20844: F, t20848: F, t20850: F, t20852: F, t20854: F) -> F {
    let t23198 = t20816 - t20819 + t20826 - t20829 + t20832 - t20835 - t20837 - t20840 - t20844 + t20848 - t20850 + t20852 + t20854;
    t23198
}
