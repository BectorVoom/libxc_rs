//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2819;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta747<F: Float>(t2435: F, t2859: F, t273: F, t270: F, t276: F, t39484: F, t9303: F, t931: F, t2922: F, t275: F, t2925: F, t41306: F, t2866: F, t2923: F) -> (F, F, F, F, F, F, F, F, F) {
        let t41363 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2819::<F>(t2435, t2859);
        let (t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41578) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2820::<F>(t273, t270, t276, t39484, t9303, t931, t2922, t275, t2925, t41306, t2866, t2923);
    (t41363, t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41578)
}
