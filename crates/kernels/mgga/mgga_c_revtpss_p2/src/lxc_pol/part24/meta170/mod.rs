//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta170 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk837;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk838;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk839;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk840;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta170<F: Float>(t247: F, t6678: F, t1264: F, t6425: F, t1774: F, t1794: F, t1250: F, t3720: F, t1222: F, t1261: F, t1782: F, t1808: F, t3657: F, t3684: F, t3718: F, t464: F, t5358: F, t5363: F, t5366: F, t5373: F, t5379: F, t5381: F, t5391: F, t6653: F, t6659: F, t6663: F, t6667: F, t6673: F, t6651: F, t225: F, t494: F, t1828: F, t3737: F, t1280: F, t6573: F, t1287: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6679, t6683, t6688, t6689, t6690) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk837::<F>(t247, t6678, t1264, t6425, t1774, t1794, t1250, t3720);
        let t6694 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk838::<F>(t1222, t1261, t1782, t1808, t3657, t3684, t3718, t464, t5358, t5363, t5366, t5373, t5379, t5381, t5391, t6653, t6659, t6663, t6667, t6673, t6679, t6683, t6690);
        let t6695 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk839::<F>(t6651, t6694);
        let (t6697, t6702) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk840::<F>(t225, t494, t6695, t1828);
        let (t6703, t6714, t6717) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk841::<F>(t3737, t6702, t1280, t6573, t1287, t6688);
    (t6679, t6683, t6688, t6689, t6690, t6695, t6697, t6702, t6703, t6714, t6717)
}
