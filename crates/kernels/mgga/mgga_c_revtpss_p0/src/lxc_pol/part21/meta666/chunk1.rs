//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2465/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465<F: Float>(t225: F, t42059: F, t11675: F, t11711: F, t11666: F, t11710: F, t4899: F, t11262: F, t3127: F, t3129: F, t11630: F, t11633: F, t3172: F) -> (F, F, F, F, F) {
    let t43154 = t42059 * t225;
    let t43169 = t11675 * t11711;
    let t43172 = t4899 * t11710 * t11666;
    let t43204 = t3127 * t11262 * t3129;
    let t43211 = t11630 * t3172 * t11633;
    (t43154, t43169, t43172, t43204, t43211)
}
