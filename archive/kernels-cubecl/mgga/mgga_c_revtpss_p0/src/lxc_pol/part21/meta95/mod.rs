//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk652;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk653;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk654;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta95<F: Float>(t661: F, t2357: F, t2256: F, t108: F, t101: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F, t114: F, t655: F, t2335: F, t2336: F, t2341: F, t69: F, t508: F, t200: F, t45: F, t2251: F, t2258: F, t78: F, t202: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2358, t2359, t2362, t2363, t2366) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk652::<F>(t661, t2357, t2256, t108, t101, t105, t2344, t2351, t2354, t656, t659, t97);
        let (t2367, t2371) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk653::<F>(t114, t2366, t655, t2335, t2336, t2341, t69);
        let (t2372, t2375) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk654::<F>(t2371, t508, t200);
        let (t2381, t2382) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk655::<F>(t45, t2251, t2258, t2375, t78, t202, zeta_threshold);
    (t2358, t2359, t2362, t2363, t2366, t2367, t2371, t2372, t2375, t2381, t2382)
}
