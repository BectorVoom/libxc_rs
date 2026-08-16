//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1691;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta440(t3667: f64, t5362: f64, t1789: f64, t371: f64, t676: f64, t1235: f64, t1769: f64, t3565: f64, t225: f64, t480: f64, t1803: f64, t3650: f64, t16708: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12678: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17301, t17303, t17304, t17306, t17307, t17308, t17311) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1691(t3667, t5362, t1789, t371, t676, t1235, t1769, t3565, t225, t480, t1803, t3650);
        let t17330 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1692(t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12678, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t17301, t17303, t17304, t17306, t17307, t17308, t17311, t17330)
}
