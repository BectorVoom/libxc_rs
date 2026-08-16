//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1742/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1742<F: Float>(t1277: F, t1774: F, t3790: F, t1204: F, t1811: F, t1211: F, t16750: F, t1209: F, t5412: F, t1828: F, t3568: F, t1294: F, t5497: F) -> (F, F, F, F, F, F) {
    let t18084 = t1277 * t1774 * t3790;
    let t18087 = t1204 * t1811;
    let t18090 = t1211 * t16750;
    let t18097 = t1209 * t5412;
    let t18102 = t1828 * t3568;
    let t18103 = t1277 * t18102;
    let t18108 = t5497 * t1294;
    (t18084, t18087, t18090, t18097, t18103, t18108)
}
