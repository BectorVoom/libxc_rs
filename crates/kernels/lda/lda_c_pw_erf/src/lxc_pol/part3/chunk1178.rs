//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1178/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1178<F: Float>(t185: F, t5128: F, t514: F, t511: F, t5184: F, t12794: F, t2193: F, t12797: F, t1446: F, t4938: F, t1392: F, t1440: F, t2098: F, t3675: F, t519: F) -> (F, F, F, F, F, F) {
    let t13883 = t185 * t514 * t5128;
    let t13884 = F::new(4.0) / F::new(15.0) * t13883;
    let t13885 = t511 * t5184;
    let t13886 = F::new(8.0) / F::new(15.0) * t13885;
    let t13888 = F::new(4.0) / F::new(5.0) * t12794 * t2193;
    let t13890 = F::new(8.0) / F::new(5.0) * t12797 * t2193;
    let t13892 = F::new(12.0) / F::new(5.0) * t1446 * t4938;
    let t13897 = F::new(12.0) / F::new(5.0) * t519 * t1440 * t3675 * t2098 * t1392;
    (t13884, t13886, t13888, t13890, t13892, t13897)
}
