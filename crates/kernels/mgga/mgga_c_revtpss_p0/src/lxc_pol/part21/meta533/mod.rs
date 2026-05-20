//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2183;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2184;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta533<F: Float>(t3362: F, t4186: F, t606: F, t3360: F, t128: F, t2258: F, t5046: F, t2251: F, t1120: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16724, t16725, t16726, t16727) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2183::<F>(t3362, t4186, t606, t3360, t128);
        let (t16729, t16730, t16731) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2184::<F>(t2258, t5046, t3360, t128);
        let (t16733, t16734, t16735) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2185::<F>(t2251, t5046, t1120, t128);
    (t16724, t16725, t16726, t16727, t16729, t16730, t16731, t16733, t16734, t16735)
}
