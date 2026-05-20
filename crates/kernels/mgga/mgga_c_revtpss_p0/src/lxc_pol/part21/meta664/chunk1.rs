//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2461/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461<F: Float>(t12004: F, t3111: F, t1011: F, t11165: F, t15987: F, t11156: F, t15993: F, t11692: F, t11922: F, t4899: F, t1086: F, t11213: F, t3090: F) -> (F, F, F, F, F) {
    let t43019 = t12004 * t3111;
    let t43029 = t1011 * t15987 * t11165;
    let t43032 = t1011 * t15993 * t11156;
    let t43035 = t4899 * t11922 * t11692;
    let t43038 = t11213 * t1086 * t3090;
    (t43019, t43029, t43032, t43035, t43038)
}
