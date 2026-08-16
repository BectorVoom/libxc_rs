//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta69 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk451;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk452;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk453;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk454;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta69(t1419: f64, t225: f64, t561: f64, t213: f64, t555: f64, t560: f64, t545: f64, t869: f64, t689: f64, t546: f64, t786: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1420 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk451(t1419, t225);
        let (t1421, t1424) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk452(t1420, t561, t213, t555);
        let (t1425, t1426, t1427) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk453(t560, t225);
        let (t1428, t1429, t1431, t1432) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk454(t545, t555, t869, t689, t546, t786);
        let t1433 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk455(t555, t72);
    (t1420, t1421, t1424, t1425, t1426, t1427, t1428, t1429, t1431, t1432, t1433)
}
