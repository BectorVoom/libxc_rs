//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1252/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1252<F: Float>(t4763: F, t6958: F, t6965: F, t2178: F, t6198: F, t15579: F, t2183: F, t12299: F, t2558: F, t4738: F, t6917: F, t2153: F, t6205: F) -> (F, F, F, F, F, F, F) {
    let t22432 = F::new(8.0) / F::new(5.0) * t4763 * t6958;
    let t22434 = F::new(8.0) / F::new(5.0) * t4763 * t6965;
    let t22436 = F::new(8.0) / F::new(15.0) * t6198 * t2178;
    let t22438 = F::new(4.0) / F::new(5.0) * t15579 * t2183;
    let t22440 = F::new(8.0) / F::new(5.0) * t12299 * t2558;
    let t22442 = F::new(8.0) / F::new(5.0) * t4738 * t6917;
    let t22444 = F::new(8.0) / F::new(15.0) * t6205 * t2153;
    (t22432, t22434, t22436, t22438, t22440, t22442, t22444)
}
