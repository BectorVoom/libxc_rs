//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 737/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk737<F: Float>(t1629: F, t2106: F, t137: F, t132: F, t1636: F, t831: F, t4838: F, t4843: F, t4846: F, t4939: F, t4943: F, t4947: F, t4950: F, t4952: F, t4956: F, t4958: F, t4960: F, t4962: F, t4964: F) -> (F, F, F, F, F) {
    let t4965 = t2106 * t1629;
    let t4966 = t137 * t4965;
    let t4968 = t132 * t4966 / F::new(30.0);
    let t4970 = F::new(2.0) / F::new(45.0) * t831 * t1636;
    let t4971 = t4838 - t4843 + t4846 - t4939 - t4943 - t4947 - t4950 - t4952 - t4956 - t4958 - t4960 - t4962 - t4964 - t4968 - t4970;
    (t4965, t4966, t4968, t4970, t4971)
}
