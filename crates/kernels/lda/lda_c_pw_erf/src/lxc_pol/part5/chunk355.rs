//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 355/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk355<F: Float>(t1432: F, t248: F, t256: F, t640: F, t656: F, t458: F, t646: F, t188: F, t22: F) -> (F, F, F, F, F) {
    let t1433 = t248 * t1432;
    let t1435 = t1433 * t256 / F::new(3.0);
    let t1436 = t640 * t656;
    let t1439 = F::new(0.033245444444444446) * t458 * t646;
    let t1440 = t22 * t188;
    (t1433, t1435, t1436, t1439, t1440)
}
