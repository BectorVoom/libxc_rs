//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2396/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2396<F: Float>(t1269: F, t1770: F, t1214: F, t5497: F, t1277: F, t1211: F, t17345: F, t1811: F, t3555: F) -> (F, F, F, F, F) {
    let t18005 = t1770 * t1269;
    let t18018 = t5497 * t1214;
    let t18019 = t1277 * t18018;
    let t18030 = t1211 * t17345;
    let t18037 = t3555 * t1811;
    (t18005, t18018, t18019, t18030, t18037)
}
