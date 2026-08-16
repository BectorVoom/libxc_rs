//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1017/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1017<F: Float>(t225: F, t494: F, t5412: F, t1811: F, t460: F, t1214: F, t1828: F, t1277: F, t1294: F, t3737: F, t1284: F, t1770: F) -> (F, F, F, F, F) {
    let t5414 = t5412 * t225 * t494;
    let t5417 = t460 * t1811;
    let t5422 = t1828 * t1214;
    let t5423 = t1277 * t5422;
    let t5428 = t1828 * t1294;
    let t5429 = t3737 * t5428;
    let t5436 = t1770 * t1284;
    (t5414, t5417, t5423, t5429, t5436)
}
