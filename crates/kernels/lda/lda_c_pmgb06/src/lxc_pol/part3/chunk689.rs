//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 689/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk689<F: Float>(t5005: F, t5038: F, t465: F, t137: F, t132: F, t1554: F, t843: F, t161: F, t1555: F, t831: F, t1548: F, t802: F, t1547: F, t814: F, t2998: F, t3007: F, t4070: F, t4079: F, t4082: F, t4089: F, t4091: F, t4973: F, t4977: F, t4981: F, t4983: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5039 = t5005 + t5038;
    let t5040 = t465 * t5039;
    let t5041 = t137 * t5040;
    let t5043 = t132 * t5041 / 30.0;
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5046 = t5045 / 135.0;
    let t5047 = t831 * t1555;
    let t5048 = t5047 / 135.0;
    let t5049 = t802 * t1548;
    let t5050 = t5049 / 135.0;
    let t5051 = t1547 * t814;
    let t5052 = t132 * t5051;
    let t5053 = t5052 / 135.0;
    let t5054 = 2.0 / 45.0 * t2998;
    let t5057 = -t4973 - t4977 - t4981 - t4983 - t5043 - t5046 - t5048 - t5050 - t5053 - t5054 + t3007 + t4070 + t4079 + t4082 + t4089 / 3.0 + 0.06077777777777778 * t4091;
    (t5039, t5040, t5041, t5043, t5044, t5046, t5048, t5050, t5051, t5053, t5054, t5057)
}
