//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1797;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1798;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1799;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta494<F: Float>(t114: F, t28034: F, t25825: F, t26148: F, t28037: F, t28039: F, t1312: F, t13426: F, t1518: F, t18227: F, t2055: F, t2322: F, t26399: F, t27123: F, t28219: F, t28652: F, t28653: F, t28658: F, t4248: F, t4292: F, t5523: F, t670: F, t7359: F, t7373: F, t7889: F, t7983: F, t1843: F, t118: F, t1502: F, t1911: F, t2052: F, t2056: F, t2089: F, t25082: F, t28196: F, t28287: F, t28586: F, t28588: F, t4246: F, t5517: F, t569: F, t651: F, t671: F, t7357: F, t7367: F, t7474: F, t7484: F, t1310: F, t7315: F, t8108: F, t13648: F, t2107: F, t508: F, t22496: F, t26405: F, t5542: F, t7536: F, t1453: F, t2014: F, t4254: F, t4293: F, t4297: F, t649: F, t7378: F, t7969: F, t7984: F, t8065: F, t8075: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t28683 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1796::<F>(t114, t28034, t25825, t26148, t28037, t28039);
        let t28686 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1797::<F>(t1312, t13426, t1518, t18227, t2055, t2322, t26399, t27123, t28219, t28652, t28653, t28658, t28683, t4248, t4292, t5523, t670, t7359, t7373, t7889, t7983);
        let (t28696, t28699) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1798::<F>(t1843, t7373, t118, t13426, t1502, t18227, t1911, t2052, t2056, t2089, t25082, t28196, t28287, t28586, t28588, t28653, t28686, t4246, t4248, t5517, t569, t651, t671, t7357, t7367, t7474, t7484);
        let (t28704, t28707, t28709, t28711, t28718, t28727) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1799::<F>(t1310, t7983, t7315, t8108, t13648, t2107, t28683, t508, t22496, t26405, t5542, t7536);
        let t28729 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1800::<F>(t1310, t1453, t2014, t2322, t25082, t28652, t28704, t28707, t28709, t28711, t28718, t28727, t4248, t4254, t4293, t4297, t508, t649, t651, t7359, t7378, t7969, t7984, t8065, t8075);
    (t28683, t28686, t28696, t28699, t28704, t28707, t28709, t28711, t28718, t28727, t28729)
}
