//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1929;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1930;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta377<F: Float>(t10389: F, t1469: F, t2299: F, t4186: F, t10398: F, t2306: F, t13312: F, t2251: F, t2258: F, t4227: F, t4232: F, t606: F, t633: F, t637: F, t77: F, t70: F) -> (F, F, F, F, F, F) {
        let (t13368, t13378, t13388) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1929::<F>(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1930::<F>(t13388, t77, t1469, t2258);
        let (t13393, t13396) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1931::<F>(t13392, t70, t4186, t606);
    (t13368, t13378, t13389, t13392, t13393, t13396)
}
