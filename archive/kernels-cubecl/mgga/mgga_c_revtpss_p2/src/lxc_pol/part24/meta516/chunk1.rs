//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1537/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1537<F: Float>(t1011: F, t15987: F, t23503: F, t19773: F, t4845: F, t140: F, t23868: F, t11922: F, t23930: F, t4892: F, t11710: F, t23903: F, t4899: F) -> (F, F, F, F, F) {
    let t79944 = t1011 * t15987 * t23503;
    let t79946 = t19773 * t4845;
    let t79957 = t1011 * t140 * t23868;
    let t80038 = t4892 * t11922 * t23930;
    let t80113 = t4899 * t11710 * t23903;
    (t79944, t79946, t79957, t80038, t80113)
}
