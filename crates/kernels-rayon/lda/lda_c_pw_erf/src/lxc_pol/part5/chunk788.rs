//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 788/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk788(t4733: f64, t4734: f64, t4790: f64, t4793: f64, t4797: f64, t4836: f64, t4879: f64, t4891: f64, t4905: f64, t4917: f64, t4935: f64, t6678: f64, t6680: f64, t6681: f64, t6684: f64, t6687: f64) -> f64 {
    let t7263 = t6678 + t4733 + 16.0_f64 / 3.0_f64 * t4734 - t4790 - t4793 + t4797 - t4836 + t4879 + t4891 - t6680 + t4905 + t6681 + t4917 - t4935 - t6684 - t6687;
    t7263
}
