//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta748<F: Float>(t11384: F, t910: F, t275: F, t2872: F, t2922: F, t41245: F, t41306: F, t315: F, t41235: F, t11449: F, t941: F, t2941: F, t2966: F, t302: F) -> (F, F, F, F, F, F, F) {
        let (t41583, t41588, t41592, t41610, t41658, t41662, t41667) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2821::<F>(t11384, t910, t275, t2872, t2922, t41245, t41306, t315, t41235, t11449, t941, t2941, t2966, t302);
    (t41583, t41588, t41592, t41610, t41658, t41662, t41667)
}
