//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1653;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1654;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta338<F: Float>(t290: F, t2925: F, t11300: F, t11385: F, t3022: F, t3030: F, t3034: F, t3006: F, t3011: F, t4733: F, t981: F, t2935: F, t945: F, t2967: F, t941: F, t2966: F, t307: F, t302: F, t2944: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11387, t11388, t11390, t11392, t11394, t11396, t11398, t11399) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1653::<F>(t290, t2925, t11300, t11385, t3022, t3030, t3034, t3006, t3011, t4733, t981, t2935, t945);
        let (t11404, t11408, t11409) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1654::<F>(t2967, t941, t2966, t307, t302);
        let t11410 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1655::<F>(t2944, t953);
    (t11387, t11388, t11390, t11392, t11394, t11396, t11398, t11399, t11404, t11408, t11409, t11410)
}
