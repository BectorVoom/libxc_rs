//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk832;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk833;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk834;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta168<F: Float>(t3611: F, t6629: F, t1042: F, t1469: F, t3628: F, t5351: F, t3626: F, t482: F, t6587: F, t371: F, t372: F, t1235: F, t1247: F, t1791: F, t1797: F, t3600: F, t3610: F, t3625: F, t3671: F, t3711: F, t484: F, t5254: F, t5256: F, t5266: F, t5274: F, t5293: F, t5323: F, t5327: F, t6595: F, t6598: F, t6602: F, t6611: F, t6619: F, t6625: F, t6631: F) -> (F, F, F, F, F, F, F, F) {
        let (t6634, t6635) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk832::<F>(t3611, t6629, t1042);
        let (t6638, t6639) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk833::<F>(t1469, t3628, t5351);
        let t6640 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk834::<F>(t3626, t6639);
        let (t6645, t6647, t6651) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk835::<F>(t482, t6587, t371, t372, t1235, t1247, t1791, t1797, t3600, t3610, t3625, t3671, t3711, t484, t5254, t5256, t5266, t5274, t5293, t5323, t5327, t6595, t6598, t6602, t6611, t6619, t6625, t6631, t6635, t6640);
    (t6634, t6635, t6638, t6639, t6640, t6645, t6647, t6651)
}
