//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 873/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk873<F: Float>(t3937: F, t5351: F, t3037: F, t3922: F, t449: F, t556: F, t1308: F, t3889: F, t545: F, t848: F, t464: F, t1219: F, t1658: F, t5384: F, t871: F, t3888: F, t5385: F) -> (F, F, F, F, F, F, F) {
    let t15106 = t3937 * t5351;
    let t15110 = t3922 * t449 * t556 * t3037;
    let t15112 = t1308 * t3889;
    let t15115 = t848 * t545;
    let t15116 = t15115 * t464;
    let t15126 = t5384 * t1219 * t1658 * t871;
    let t15129 = t5384 * t5385 * t3888;
    (t15106, t15110, t15112, t15115, t15116, t15126, t15129)
}
