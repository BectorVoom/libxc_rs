//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta61 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk389;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk390;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk391;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk392;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk393;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta61(t1544: f64, t832: f64, t1553: f64, t227: f64, t229: f64, t231: f64, t828: f64, t827: f64, t855: f64, t1549: f64, t797: f64, t799: f64, t812: f64, t819: f64, t825: f64, t848: f64, t851: f64, t225: f64, t257: f64, t879: f64, t234: f64, t213: f64, t820: f64, t873: f64, t878: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1555, t1558) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk389(t1544, t832, t1553, t227, t229);
        let t1559 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk390(t1558, t231);
        let (t1561, t1565, t1568) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk391(t1559, t828, t827, t1544, t855, t1549, t797, t799, t812, t819, t825, t848, t851);
        let t1569 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk392(t1568, t225);
        let (t1570, t1573, t1576, t1579) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk393(t1569, t257, t1559, t879, t1568, t234, t213, t820, t873, t878);
        let t1580 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk394(t1579, t868);
    (t1555, t1558, t1559, t1561, t1565, t1568, t1569, t1570, t1573, t1576, t1579, t1580)
}
