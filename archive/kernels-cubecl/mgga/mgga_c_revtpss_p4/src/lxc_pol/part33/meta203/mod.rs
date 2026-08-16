//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk943;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk944;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk945;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk946;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk947;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk948;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk949;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta203<F: Float>(t1715: F, t3634: F, t247: F, t1261: F, t1260: F, t1785: F, t3670: F, t3719: F, t5230: F, t1802: F, t369: F, t475: F, t467: F, t1264: F, t5056: F, t3629: F, t5351: F, t3626: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t5378 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk943::<F>(t1715, t3634, t247);
        let (t5379, t5381) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk944::<F>(t1261, t5378, t1260, t1785);
        let t5384 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk945::<F>(t1260, t3670);
        let t5386 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk946::<F>(t3719, t5230, t247);
        let (t5389, t5390) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk947::<F>(t1802, t369, t475);
        let t5391 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk948::<F>(t467, t5390);
        let t5397 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk949::<F>(t1264, t5056, t247);
        let (t5401, t5402) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk950::<F>(t3629, t5351, t3626);
    (t5378, t5379, t5381, t5384, t5386, t5389, t5390, t5391, t5397, t5401, t5402)
}
