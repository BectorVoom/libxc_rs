//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 641/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk641<F: Float>(t1476: F, t2146: F, t213: F, t473: F, t34: F, t581: F, t1124: F, t573: F, t2152: F, t571: F, t1446: F, t2143: F) -> (F, F, F, F, F, F, F) {
    let t4891 = F::new(16.0) / F::new(135.0) * t2146 * t1476;
    let t4892 = t473 * t213;
    let t4893 = t581 * t34;
    let t4900 = t1124 * t573;
    let t4901 = t4900 * t2152;
    let t4902 = t571 * t4901;
    let t4905 = F::new(16.0) / F::new(135.0) * t1446 * t2143;
    (t4891, t4892, t4893, t4900, t4901, t4902, t4905)
}
