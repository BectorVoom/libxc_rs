//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2551/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2551<F: Float>(t1053: F, t15670: F, t11262: F, t3127: F, t4824: F, t11671: F, t4954: F, t11998: F, t15822: F, t1086: F, t15669: F, t3090: F) -> (F, F, F, F, F) {
    let t54404 = t15670 * t1053;
    let t54414 = t3127 * t11262 * t4824;
    let t54471 = t4954 * t11671;
    let t54492 = t15822 * t11998;
    let t54500 = t15669 * t1086 * t3090;
    (t54404, t54414, t54471, t54492, t54500)
}
