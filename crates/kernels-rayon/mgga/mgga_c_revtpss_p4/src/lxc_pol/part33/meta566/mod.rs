//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta566(t1785: f64, t8184: f64, t2137: f64, t6593: f64, t467: f64, t1782: f64, t1791: f64, t1797: f64, t26824: f64, t26870: f64, t26877: f64, t29010: f64, t29062: f64, t29072: f64, t29077: f64, t29086: f64, t29089: f64, t484: f64, t6611: f64, t6647: f64, t6653: f64, t6659: f64, t6663: f64, t6673: f64, t6683: f64, t6690: f64, t7607: f64, t7613: f64, t7624: f64, t30805: f64, t225: f64, t494: f64, t1794: f64, t8201: f64, t1287: f64, t8197: f64, t3783: f64, t6628: f64, t7660: f64, t1770: f64, t2144: f64, t26889: f64, t26895: f64, t26906: f64, t26922: f64, t26949: f64, t29136: f64, t29141: f64, t29275: f64, t30736: f64, t30740: f64, t30744: f64, t30748: f64, t30752: f64, t30758: f64, t30764: f64, t30768: f64, t30772: f64, t460: f64, t6564: f64, t7636: f64, t7643: f64, t7651: f64, t8192: f64, t8198: f64, t8202: f64, t8205: f64, t8209: f64, t8217: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30812, t30815, t30816, t30839) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1967(t1785, t8184, t2137, t6593, t467, t1782, t1791, t1797, t26824, t26870, t26877, t29010, t29062, t29072, t29077, t29086, t29089, t484, t6611, t6647, t6653, t6659, t6663, t6673, t6683, t6690, t7607, t7613, t7624);
        let t30840 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1968(t30805, t30839);
        let (t30842, t30849, t30850, t30853, t30854, t30860, t30865) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1969(t225, t30840, t494, t1794, t8201, t1287, t8197, t3783, t6628, t7660, t1770, t2144, t26889, t26895, t26906, t26922, t26949, t29136, t29141, t29275, t30736, t30740, t30744, t30748, t30752, t30758, t30764, t30768, t30772, t460, t6564, t7636, t7643, t7651, t8192, t8198, t8202, t8205, t8209, t8217);
    (t30812, t30815, t30816, t30840, t30842, t30849, t30850, t30853, t30854, t30860, t30865)
}
