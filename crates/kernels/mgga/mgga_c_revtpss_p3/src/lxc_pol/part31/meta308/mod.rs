//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1304;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta308<F: Float>(t10671: F, t813: F, t2689: F, t2694: F, t243: F, t247: F, t9949: F, t237: F, t236: F, t9646: F, t9721: F, t268: F, t207: F, t242: F, t240: F, t72: F, t136: F, t2476: F, t2482: F, t596: F, t849: F, t2677: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10673, t10678, t10685, t10687, t10688, t10690) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1304::<F>(t10671, t813, t2689, t2694, t243, t247, t9949, t237, t236, t9646, t9721, t268);
        let (t10692, t10698, t10703, t10716, t10717) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1305::<F>(t10688, t10690, t207, t242, t240, t72, t136, t2476, t2482, t596, t849, t2677);
    (t10673, t10678, t10685, t10687, t10690, t10692, t10698, t10703, t10716, t10717)
}
