//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1358;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta414<F: Float>(t12046: F, t15905: F, t994: F, t1014: F, t11150: F, t221: F, t345: F, t346: F, t624: F, t1065: F, t215: F, t373: F, t675: F, t828: F, t11238: F, t196: F, t342: F, t11626: F, t358: F, t3145: F, t365: F, t360: F, t3153: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42690, t42731, t42745, t42778, t42792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1357::<F>(t12046, t15905, t994, t1014, t11150, t221, t345, t346, t624, t1065, t215, t373, t675);
        let (t42793, t42859) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1358::<F>(t42792, t828, t11238, t196);
        let (t42860, t42862, t42865, t42866, t42868, t42871) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1359::<F>(t342, t42859, t11626, t358, t3145, t365, t360, t3153);
    (t42690, t42731, t42745, t42778, t42793, t42859, t42860, t42862, t42865, t42866, t42868, t42871)
}
