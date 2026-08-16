//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1233;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1234;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta281<F: Float>(t572: F, t7953: F, t1469: F, t1479: F, t61: F, t6971: F, t7571: F, t72: F, t1927: F, t2122: F, t7719: F, t5: F, t265: F, t393: F, t1923: F, t2123: F, t7566: F, t7702: F, t7706: F, t7709: F, t117: F, t1518: F, t2163: F, t7855: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7955, t8142, t8143, t8144) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1233::<F>(t572, t7953, t1469, t1479, t61, t6971, t7571, t72, t1927);
        let t8147 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1234::<F>(t2122, t7719);
        let (t8151, t8152, t8158, t8161) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1235::<F>(t5, t265, t393, t1923, t2123, t7566, t7702, t7706, t7709, t8144, t8147, t117, t1518, t2163, t7855);
    (t7955, t8142, t8143, t8144, t8147, t8151, t8152, t8158, t8161)
}
