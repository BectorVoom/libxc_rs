//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3105/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3105<F: Float>(t1045: F, t606: F, t11937: F, t15671: F, t11262: F, t3127: F, t4824: F, t1065: F, t15648: F, t15772: F, t3188: F, t1063: F, t16195: F, t3172: F) -> (F, F, F, F, F, F) {
    let t54397 = t1045 * t606;
    let t54407 = t15671 * t11937;
    let t54414 = t3127 * t11262 * t4824;
    let t54419 = t1065 * t15648;
    let t54432 = t3188 * t15772;
    let t54435 = t1063 * t3172 * t16195;
    (t54397, t54407, t54414, t54419, t54432, t54435)
}
