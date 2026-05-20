//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk834;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk835;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk836;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk837;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk838;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk839;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta170<F: Float>(t4533: F, t868: F, t1580: F, t213: F, t2437: F, t2443: F, t2446: F, t2449: F, t2460: F, t2462: F, t2468: F, t2473: F, t257: F, t2765: F, t4323: F, t4326: F, t4470: F, t4474: F, t4478: F, t4482: F, t4487: F, t865: F, t887: F, t198: F, t205: F, t1544: F, t262: F, t1583: F, t892: F, t2404: F, t2411: F, t1940: F, t207: F, t2403: F, t2621: F, t2628: F, t2632: F, t4316: F, t4343: F, t4394: F, t4396: F, t4397: F, t4400: F, t4405: F, t4406: F, t765: F, t775: F, t890: F, t4314: F, t2: F, t265: F, t580: F, t1593: F, t689: F, t1469: F, t2852: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4534 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk834::<F>(t4533, t868);
        let t4537 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk835::<F>(t1580, t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2765, t4323, t4326, t4470, t4474, t4478, t4482, t4487, t4534, t865, t887);
        let t4541 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk836::<F>(t198, t205);
        let (t4542, t4546, t4556, t4559) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk837::<F>(t1544, t262, t1583, t892, t2404, t2411, t1940, t198, t207, t2403, t2621, t2628, t2632, t4316, t4343, t4394, t4396, t4397, t4400, t4405, t4406, t4537, t4541, t765, t775, t890);
        let t4560 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk838::<F>(t4314, t4559);
        let (t4568, t4571) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk839::<F>(t2, t265, t580, t1593, t689);
        let (t4573, t4574) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk840::<F>(t1469, t2852, t606);
    (t4534, t4537, t4541, t4542, t4546, t4556, t4560, t4568, t4571, t4573, t4574)
}
