//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2380;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta622<F: Float>(t10733: F, t9775: F, t10716: F, t10741: F, t10665: F, t243: F, t231: F, t2661: F, t2662: F, t10737: F, t2652: F, t212: F, t2237: F, t225: F, t816: F, t2665: F, t10627: F, t10697: F, t236: F, t807: F, t10689: F, t237: F, t247: F, t10709: F, t10744: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40475, t40477, t40479, t40482, t40484, t40488) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2380::<F>(t10733, t9775, t10716, t10741, t10665, t243, t231, t2661, t2662, t10737, t2652, t212, t2237, t225, t816);
        let (t40489, t40503, t40507, t40509) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2381::<F>(t2665, t40488, t10627, t10697, t236, t807, t10689, t237, t247, t10709, t10744, t808);
    (t40475, t40477, t40479, t40482, t40484, t40488, t40489, t40503, t40507, t40509)
}
