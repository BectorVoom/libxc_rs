//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1767;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1768;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta494(t1310: f64, t7983: f64, t7315: f64, t8108: f64, t13648: f64, t2107: f64, t28683: f64, t508: f64, t22496: f64, t26405: f64, t5542: f64, t7536: f64, t1453: f64, t2014: f64, t2322: f64, t25082: f64, t28652: f64, t4248: f64, t4254: f64, t4293: f64, t4297: f64, t649: f64, t651: f64, t7359: f64, t7378: f64, t7969: f64, t7984: f64, t8065: f64, t8075: f64, t2089: f64, t4292: f64, t670: f64, t1518: f64, t7474: f64, t1519: f64, t26399: f64, t28658: f64, t4257: f64, t7235: f64, t7374: f64, t7537: f64, t7539: f64, t7732: f64, t7898: f64, t7978: f64, t7988: f64, t8111: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28704, t28707, t28709, t28711, t28718, t28727) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1767(t1310, t7983, t7315, t8108, t13648, t2107, t28683, t508, t22496, t26405, t5542, t7536);
        let t28729 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1768(t1310, t1453, t2014, t2322, t25082, t28652, t28704, t28707, t28709, t28711, t28718, t28727, t4248, t4254, t4293, t4297, t508, t649, t651, t7359, t7378, t7969, t7984, t8065, t8075);
        let (t28734, t28737, t28750, t28759) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1769(t2089, t4292, t670, t8065, t1518, t7474, t1519, t2322, t26399, t28658, t4254, t4257, t651, t7235, t7359, t7374, t7537, t7539, t7732, t7898, t7978, t7988, t8111);
    (t28704, t28707, t28709, t28711, t28718, t28727, t28729, t28734, t28737, t28750, t28759)
}
