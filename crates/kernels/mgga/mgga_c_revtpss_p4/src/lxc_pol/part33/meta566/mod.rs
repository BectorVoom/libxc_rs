//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1967;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1968;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta566<F: Float>(t1785: F, t8184: F, t2137: F, t6593: F, t467: F, t1782: F, t1791: F, t1797: F, t26824: F, t26870: F, t26877: F, t29010: F, t29062: F, t29072: F, t29077: F, t29086: F, t29089: F, t484: F, t6611: F, t6647: F, t6653: F, t6659: F, t6663: F, t6673: F, t6683: F, t6690: F, t7607: F, t7613: F, t7624: F, t30805: F, t225: F, t494: F, t1794: F, t8201: F, t1287: F, t8197: F, t3783: F, t6628: F, t7660: F, t1770: F, t2144: F, t26889: F, t26895: F, t26906: F, t26922: F, t26949: F, t29136: F, t29141: F, t29275: F, t30736: F, t30740: F, t30744: F, t30748: F, t30752: F, t30758: F, t30764: F, t30768: F, t30772: F, t460: F, t6564: F, t7636: F, t7643: F, t7651: F, t8192: F, t8198: F, t8202: F, t8205: F, t8209: F, t8217: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t30812, t30815, t30816, t30839) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1967::<F>(t1785, t8184, t2137, t6593, t467, t1782, t1791, t1797, t26824, t26870, t26877, t29010, t29062, t29072, t29077, t29086, t29089, t484, t6611, t6647, t6653, t6659, t6663, t6673, t6683, t6690, t7607, t7613, t7624);
        let t30840 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1968::<F>(t30805, t30839);
        let (t30842, t30849, t30850, t30853, t30854, t30860, t30865) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1969::<F>(t225, t30840, t494, t1794, t8201, t1287, t8197, t3783, t6628, t7660, t1770, t2144, t26889, t26895, t26906, t26922, t26949, t29136, t29141, t29275, t30736, t30740, t30744, t30748, t30752, t30758, t30764, t30768, t30772, t460, t6564, t7636, t7643, t7651, t8192, t8198, t8202, t8205, t8209, t8217);
    (t30812, t30815, t30816, t30840, t30842, t30849, t30850, t30853, t30854, t30860, t30865)
}
