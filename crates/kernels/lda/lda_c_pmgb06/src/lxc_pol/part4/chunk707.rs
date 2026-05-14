//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 707/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk707<F: Float>(t1499: F, t853: F, t2101: F, t486: F, t1588: F, t831: F, t2066: F, t432: F, t1629: F, t2106: F, t137: F, t132: F, t1636: F, t4838: F, t4843: F, t4846: F, t4939: F, t4943: F, t4947: F, t4950: F, t4952: F, t4956: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4958 = t1499 * t853 / 30.0;
    let t4960 = t486 * t2101 / 15.0;
    let t4962 = t831 * t1588 / 30.0;
    let t4964 = t432 * t2066 / 15.0;
    let t4965 = t2106 * t1629;
    let t4966 = t137 * t4965;
    let t4968 = t132 * t4966 / 30.0;
    let t4970 = 2.0 / 45.0 * t831 * t1636;
    let t4971 = t4838 - t4843 + t4846 - t4939 - t4943 - t4947 - t4950 - t4952 - t4956 - t4958 - t4960 - t4962 - t4964 - t4968 - t4970;
    (t4958, t4960, t4962, t4964, t4965, t4966, t4968, t4970, t4971)
}
