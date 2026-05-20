//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1742;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1743;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta489<F: Float>(t72: F, t8015: F, t686: F, t7058: F, t7064: F, t689: F, t8011: F, t25431: F, t25411: F, t786: F, t7998: F, t789: F, t231: F, t7997: F, t836: F, t7076: F, t1558: F, t7398: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28359, t28360) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1742::<F>(t72, t8015, t686);
        let (t28361, t28366, t28368) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1743::<F>(t28360, t7058, t7064, t689, t8011);
        let (t28369, t28371, t28373, t28374, t28377, t28378, t28384) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744::<F>(t25431, t28368, t25411, t786, t7998, t789, t231, t7997, t836, t7076, t1558, t7398);
    (t28359, t28360, t28361, t28366, t28368, t28369, t28371, t28373, t28374, t28377, t28378, t28384)
}
