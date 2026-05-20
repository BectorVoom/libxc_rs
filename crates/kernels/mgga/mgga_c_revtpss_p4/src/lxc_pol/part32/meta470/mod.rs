//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1697;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1698;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta470<F: Float>(t2097: F, t785: F, t1358: F, t2439: F, t2435: F, t7493: F, t26069: F, t26277: F, t26072: F, t7515: F, t116: F, t7356: F, t2106: F, t4147: F, t531: F, t7535: F, t198: F, t206: F, t2070: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26358, t26359, t26361, t26363, t26365, t26366, t26399) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1697::<F>(t2097, t785, t1358, t2439, t2435, t7493, t26069, t26277, t26072, t7515, t116, t7356);
        let t26405 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1698::<F>(t2106, t4147);
        let (t26411, t26425) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1699::<F>(t531, t7535, t198, t206, t2070);
    (t26358, t26359, t26361, t26363, t26365, t26366, t26399, t26405, t26411, t26425)
}
