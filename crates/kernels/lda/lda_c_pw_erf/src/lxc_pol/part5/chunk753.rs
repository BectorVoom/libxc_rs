//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 753/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk753<F: Float>(t2072: F, t6875: F, t5211: F, t813: F, t247: F, t6039: F, t251: F, t2462: F, t652: F, t256: F, t19: F, t2363: F) -> (F, F, F, F, F, F, F) {
    let t6877 = F::new(8.0) / F::new(15.0) * t6875 * t2072;
    let t6879 = F::new(8.0) / F::new(15.0) * t5211 * t813;
    let t6880 = t6039 * t247;
    let t6881 = t6880 * t251;
    let t6884 = t2462 * t652;
    let t6885 = t6884 * t256;
    let t6887 = t2363 * t19;
    (t6877, t6879, t6880, t6881, t6884, t6885, t6887)
}
