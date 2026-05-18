//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 371/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk371<F: Float>(t1326: F, t1328: F, t1325: F, t261: F, t50: F) -> (F, F, F, F) {
    let t1329 = t1326 * t1328;
    let t1331 = F::new(16.0) / F::new(45.0) * t1325 * t1329;
    let t1332 = t261 * t50;
    let t1333 = F::new(1.0) / t1332;
    (t1329, t1331, t1332, t1333)
}
