//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta508<F: Float>(t16668: F, t3385: F, t12227: F, t3520: F, t5180: F, t5206: F, t1196: F, t3495: F, t1189: F, t3543: F, t5192: F, t3516: F, t5197: F) -> (F, F, F, F, F, F, F, F) {
        let (t16669, t16671, t16673, t16675, t16677, t16679, t16681, t16682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2251::<F>(t16668, t3385, t12227, t3520, t5180, t5206, t1196, t3495, t1189, t3543, t5192, t3516, t5197);
    (t16669, t16671, t16673, t16675, t16677, t16679, t16681, t16682)
}
