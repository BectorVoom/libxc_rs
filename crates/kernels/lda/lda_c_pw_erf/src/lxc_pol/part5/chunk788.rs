//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 788/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk788<F: Float>(t4733: F, t4734: F, t4790: F, t4793: F, t4797: F, t4836: F, t4879: F, t4891: F, t4905: F, t4917: F, t4935: F, t6678: F, t6680: F, t6681: F, t6684: F, t6687: F) -> F {
    let t7263 = t6678 + t4733 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t4734 - t4790 - t4793 + t4797 - t4836 + t4879 + t4891 - t6680 + t4905 + t6681 + t4917 - t4935 - t6684 - t6687;
    t7263
}
