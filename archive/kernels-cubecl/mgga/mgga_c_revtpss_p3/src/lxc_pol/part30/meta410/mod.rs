//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1538;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1539;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta410<F: Float>(t15098: F, t2924: F, t1596: F, t2873: F, t2876: F, t1614: F, t2942: F, t11354: F, t1600: F, t2881: F, t11358: F, t2880: F, t4606: F, t918: F, t2889: F, t4598: F, t2897: F, t4614: F, t1606: F, t2439: F, t4580: F, t689: F, t4575: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15100, t15103, t15104, t15108, t15111, t15113) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1538::<F>(t15098, t2924, t1596, t2873, t2876, t1614, t2942, t11354, t1600, t2881, t11358, t2880, t4606);
        let (t15114, t15116, t15119, t15121, t15123, t15125) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1539::<F>(t15113, t918, t2889, t4598, t2897, t4606, t4614, t1606, t2439, t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1540::<F>(t4575, t689);
    (t15100, t15103, t15104, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15125, t15127)
}
