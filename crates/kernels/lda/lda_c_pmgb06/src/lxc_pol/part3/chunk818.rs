//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 818/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk818<F: Float>(t1580: F, t955: F, t1583: F, t1577: F, t3362: F, t405: F, t3359: F, t1414: F, t147: F, t163: F, t3338: F, t146: F, t164: F, t9712: F, t9501: F, t3347: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9954 = t955 * t1580;
    let t9956 = t955 * t1583;
    let t9958 = t955 * t1577;
    let t9960 = t405 * t3362;
    let t9962 = t405 * t3359;
    let t9967 = t147 / t163 / t1414;
    let t9974 = t405 * t3338;
    let t9981 = 0.10864197530864197 * t146 * t9712 * t164;
    let t9986 = 0.3732469135802469 * t9501;
    let t9987 = t405 * t3347;
    (t9954, t9956, t9958, t9960, t9962, t9967, t9974, t9981, t9986, t9987)
}
