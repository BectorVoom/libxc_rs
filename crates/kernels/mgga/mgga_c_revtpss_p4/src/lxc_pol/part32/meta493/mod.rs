//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta493 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1760;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1761;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1762;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1763;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1764;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1765;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta493<F: Float>(t2047: F, t28089: F, t7349: F, t7702: F, t7348: F, t7719: F, t1923: F, t2048: F, t25162: F, t26170: F, t26175: F, t26182: F, t26190: F, t26207: F, t28093: F, t28133: F, t28147: F, t28154: F, t28628: F, t6954: F, t6963: F, t7343: F, t7352: F, t7964: F, t5: F, t28621: F, t117: F, t116: F, t7968: F, t2051: F, t670: F, t114: F, t28034: F, t25825: F, t26148: F, t28037: F, t28039: F, t1312: F, t13426: F, t1518: F, t18227: F, t2055: F, t2322: F, t26399: F, t27123: F, t28219: F, t4248: F, t4292: F, t5523: F, t7359: F, t7373: F, t7889: F, t7983: F, t1843: F, t118: F, t1502: F, t1911: F, t2052: F, t2056: F, t2089: F, t25082: F, t28196: F, t28287: F, t28586: F, t28588: F, t4246: F, t5517: F, t569: F, t651: F, t671: F, t7357: F, t7367: F, t7474: F, t7484: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28635, t28638, t28640) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1760::<F>(t2047, t28089, t7349, t7702, t7348, t7719);
        let (t28641, t28649) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1761::<F>(t1923, t28640, t2048, t25162, t26170, t26175, t26182, t26190, t26207, t28093, t28133, t28147, t28154, t28628, t28635, t28638, t6954, t6963, t7343, t7352, t7702, t7964);
        let (t28651, t28652, t28653) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1762::<F>(t5, t28621, t28649, t117, t116, t7968);
        let t28658 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1763::<F>(t2051, t670);
        let t28683 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1764::<F>(t114, t28034, t25825, t26148, t28037, t28039);
        let t28686 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1765::<F>(t1312, t13426, t1518, t18227, t2055, t2322, t26399, t27123, t28219, t28652, t28653, t28658, t28683, t4248, t4292, t5523, t670, t7359, t7373, t7889, t7983);
        let (t28696, t28699) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1766::<F>(t1843, t7373, t118, t13426, t1502, t18227, t1911, t2052, t2056, t2089, t25082, t28196, t28287, t28586, t28588, t28653, t28686, t4246, t4248, t5517, t569, t651, t671, t7357, t7367, t7474, t7484);
    (t28635, t28638, t28640, t28641, t28651, t28652, t28653, t28658, t28683, t28686, t28696, t28699)
}
