//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1917;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta372(t1204: f64, t3140: f64, t3599: f64, t1242: f64, t3603: f64, t471: f64, t11249: f64, t3609: f64, t1032: f64, t3552: f64, t1246: f64, t247: f64, t3372: f64, t3634: f64, t1261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13032, t13033, t13037, t13038, t13045) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1917(t1204, t3140, t3599, t1242, t3603, t471);
        let (t13046, t13053, t13058, t13068, t13069, t13085, t13086) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1918(t11249, t13045, t3603, t13032, t3609, t1032, t3552, t1246, t247, t3372, t3634, t1261);
    (t13033, t13037, t13038, t13045, t13046, t13053, t13058, t13068, t13069, t13085, t13086)
}
