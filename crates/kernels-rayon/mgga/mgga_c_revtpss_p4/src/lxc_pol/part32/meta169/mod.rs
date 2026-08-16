//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk795;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk796;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk797;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk798;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk799;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk800;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta169(t4533: f64, t868: f64, t1580: f64, t213: f64, t2437: f64, t2443: f64, t2446: f64, t2449: f64, t2460: f64, t2462: f64, t2468: f64, t2473: f64, t257: f64, t2765: f64, t4323: f64, t4326: f64, t4470: f64, t4474: f64, t4478: f64, t4482: f64, t4487: f64, t865: f64, t887: f64, t198: f64, t205: f64, t1544: f64, t262: f64, t1583: f64, t892: f64, t2404: f64, t2411: f64, t1940: f64, t207: f64, t2403: f64, t2621: f64, t2628: f64, t2632: f64, t4316: f64, t4343: f64, t4394: f64, t4396: f64, t4397: f64, t4400: f64, t4405: f64, t4406: f64, t765: f64, t775: f64, t890: f64, t4314: f64, t2: f64, t265: f64, t580: f64, t1593: f64, t689: f64, t1469: f64, t2852: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4534 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk795(t4533, t868);
        let t4537 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk796(t1580, t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2765, t4323, t4326, t4470, t4474, t4478, t4482, t4487, t4534, t865, t887);
        let t4541 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk797(t198, t205);
        let (t4542, t4546, t4556, t4559) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk798(t1544, t262, t1583, t892, t2404, t2411, t1940, t198, t207, t2403, t2621, t2628, t2632, t4316, t4343, t4394, t4396, t4397, t4400, t4405, t4406, t4537, t4541, t765, t775, t890);
        let t4560 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk799(t4314, t4559);
        let (t4568, t4571) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk800(t2, t265, t580, t1593, t689);
        let (t4573, t4574) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk801(t1469, t2852, t606);
    (t4534, t4537, t4541, t4542, t4546, t4556, t4560, t4568, t4571, t4573, t4574)
}
