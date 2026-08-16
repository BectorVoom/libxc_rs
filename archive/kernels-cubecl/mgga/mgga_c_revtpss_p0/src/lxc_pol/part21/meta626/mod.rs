//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2388;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta626<F: Float>(t10489: F, t236: F, t807: F, t854: F, t10681: F, t2689: F, t16: F, t2236: F, t240: F, t243: F, t281: F, t39644: F, t2645: F, t775: F, t10779: F, t10786: F, t14931: F, t40583: F, t10773: F, t10811: F, t10696: F, t72: F, t245: F, t10729: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40643, t40645, t40649, t40650, t40654) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2388::<F>(t10489, t236, t807, t854, t10681, t2689, t16, t2236, t240, t243, t281, t39644);
        let (t40655, t40662, t40669, t40672, t40673, t40679) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2389::<F>(t2645, t775, t10779, t10786, t14931, t40583, t10773, t10811, t10696, t72, t245, t10729, t9775);
    (t40643, t40645, t40649, t40650, t40654, t40655, t40662, t40669, t40672, t40673, t40679)
}
