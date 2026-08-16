//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1797;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1798;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1799;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta494(t114: f64, t28034: f64, t25825: f64, t26148: f64, t28037: f64, t28039: f64, t1312: f64, t13426: f64, t1518: f64, t18227: f64, t2055: f64, t2322: f64, t26399: f64, t27123: f64, t28219: f64, t28652: f64, t28653: f64, t28658: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t7359: f64, t7373: f64, t7889: f64, t7983: f64, t1843: f64, t118: f64, t1502: f64, t1911: f64, t2052: f64, t2056: f64, t2089: f64, t25082: f64, t28196: f64, t28287: f64, t28586: f64, t28588: f64, t4246: f64, t5517: f64, t569: f64, t651: f64, t671: f64, t7357: f64, t7367: f64, t7474: f64, t7484: f64, t1310: f64, t7315: f64, t8108: f64, t13648: f64, t2107: f64, t508: f64, t22496: f64, t26405: f64, t5542: f64, t7536: f64, t1453: f64, t2014: f64, t4254: f64, t4293: f64, t4297: f64, t649: f64, t7378: f64, t7969: f64, t7984: f64, t8065: f64, t8075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t28683 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1796(t114, t28034, t25825, t26148, t28037, t28039);
        let t28686 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1797(t1312, t13426, t1518, t18227, t2055, t2322, t26399, t27123, t28219, t28652, t28653, t28658, t28683, t4248, t4292, t5523, t670, t7359, t7373, t7889, t7983);
        let (t28696, t28699) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1798(t1843, t7373, t118, t13426, t1502, t18227, t1911, t2052, t2056, t2089, t25082, t28196, t28287, t28586, t28588, t28653, t28686, t4246, t4248, t5517, t569, t651, t671, t7357, t7367, t7474, t7484);
        let (t28704, t28707, t28709, t28711, t28718, t28727) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1799(t1310, t7983, t7315, t8108, t13648, t2107, t28683, t508, t22496, t26405, t5542, t7536);
        let t28729 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1800(t1310, t1453, t2014, t2322, t25082, t28652, t28704, t28707, t28709, t28711, t28718, t28727, t4248, t4254, t4293, t4297, t508, t649, t651, t7359, t7378, t7969, t7984, t8065, t8075);
    (t28683, t28686, t28696, t28699, t28704, t28707, t28709, t28711, t28718, t28727, t28729)
}
