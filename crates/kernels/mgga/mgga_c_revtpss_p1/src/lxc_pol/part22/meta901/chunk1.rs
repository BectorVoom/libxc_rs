//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3096/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096<F: Float>(t3105: F, t4857: F, t1012: F, t43222: F, t16190: F, t3173: F, t15711: F, t3188: F, t1011: F, t15145: F, t15987: F, t15149: F) -> (F, F, F, F, F, F) {
    let t53926 = t4857 * t3105;
    let t53944 = t1012 * t43222;
    let t53948 = t16190 * t3173;
    let t53955 = t3188 * t15711;
    let t53958 = t1011 * t15987 * t15145;
    let t53961 = t1011 * t15987 * t15149;
    (t53926, t53944, t53948, t53955, t53958, t53961)
}
