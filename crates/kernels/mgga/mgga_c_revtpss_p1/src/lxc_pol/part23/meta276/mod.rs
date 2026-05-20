//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta276<F: Float>(t243: F, t816: F, t9707: F, t813: F, t2689: F, t2694: F, t247: F, t9949: F, t237: F, t236: F, t9646: F, t9721: F, t268: F, t207: F, t242: F, t240: F, t72: F, t136: F, t2476: F, t2482: F, t596: F, t849: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10673, t10678, t10687, t10688, t10689) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1491::<F>(t243, t816, t9707, t813, t2689, t2694, t247, t9949, t237, t236, t9646, t9721);
        let (t10692, t10696, t10697, t10698, t10703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1492::<F>(t10689, t268, t10688, t207, t242, t240, t72, t136, t2476);
        let t10716 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1493::<F>(t2482, t596, t849);
    (t10673, t10678, t10687, t10688, t10689, t10692, t10696, t10697, t10698, t10703, t10716)
}
