//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta250<F: Float>(t3992: F, t5609: F, t2661: F, t1414: F, t5591: F, t828: F, t1413: F, t1868: F, t547: F, t807: F, t221: F, t3979: F, t3978: F, t1885: F, t3930: F, t1353: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5610, t5611, t5614, t5617, t5618, t5619, t5622) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1104::<F>(t3992, t5609, t2661, t1414, t5591, t828, t1413, t1868, t547, t807, t221, t3979);
        let (t5623, t5625, t5627) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1105::<F>(t3978, t5622, t1885, t3930, t1353, t1868);
    (t5610, t5611, t5614, t5617, t5618, t5619, t5622, t5623, t5625, t5627)
}
