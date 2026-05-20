//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta61 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk389;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk390;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk391;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk392;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk393;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta61<F: Float>(t1544: F, t832: F, t1553: F, t227: F, t229: F, t231: F, t828: F, t827: F, t855: F, t1549: F, t797: F, t799: F, t812: F, t819: F, t825: F, t848: F, t851: F, t225: F, t257: F, t879: F, t234: F, t213: F, t820: F, t873: F, t878: F, t868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1555, t1558) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk389::<F>(t1544, t832, t1553, t227, t229);
        let t1559 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk390::<F>(t1558, t231);
        let (t1561, t1565, t1568) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk391::<F>(t1559, t828, t827, t1544, t855, t1549, t797, t799, t812, t819, t825, t848, t851);
        let t1569 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk392::<F>(t1568, t225);
        let (t1570, t1573, t1576, t1579) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk393::<F>(t1569, t257, t1559, t879, t1568, t234, t213, t820, t873, t878);
        let t1580 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk394::<F>(t1579, t868);
    (t1555, t1558, t1559, t1561, t1565, t1568, t1569, t1570, t1573, t1576, t1579, t1580)
}
