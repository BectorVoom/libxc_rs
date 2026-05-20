//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1078;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1079;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta295<F: Float>(t3153: F, t6622: F, t1263: F, t6587: F, t3172: F, t6624: F, t1247: F, t1032: F, t6564: F, t1246: F, t127: F, t371: F, t6645: F, t1235: F, t6609: F, t3671: F, t1208: F, t6563: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20800, t20809, t20816, t20817, t20819, t20820, t20842) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1078::<F>(t3153, t6622, t1263, t6587, t3172, t6624, t1247, t1032, t6564, t1246, t127, t371, t6645);
        let (t20843, t20846, t20847, t20849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1079::<F>(t1235, t20842, t127, t371, t6609, t3671, t1208, t6563);
        let t20850 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1080::<F>(t20849, t225);
    (t20800, t20809, t20816, t20817, t20819, t20820, t20842, t20843, t20846, t20847, t20849, t20850)
}
