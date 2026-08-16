//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1066 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1066<F: Float>(t1317: F, t22195: F, t48235: F, t48237: F, t48240: F, t48243: F, t46975: F, t46977: F, t46983: F, t1320: F, t22193: F, t10186: F, t198: F, t39531: F, t49544: F, t6836: F, t6930: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t73361, t73364, t73365, t73366, t73367, t73371, t73372, t73373, t73375, t73376) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3816::<F>(t1317, t22195, t48235, t48237, t48240, t48243, t46975, t46977, t46983, t1320, t22193, t10186, t198, t39531, t49544, t6836, t6930);
    (t73361, t73364, t73365, t73366, t73367, t73371, t73372, t73373, t73375, t73376)
}
