//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2736;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2737;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta711<F: Float>(t10578: F, t9863: F, t762: F, t9291: F, t2629: F, t2: F, t588: F, t2576: F, t2565: F, t701: F, t121: F, t4: F, t131: F, t268: F, t8779: F, t9282: F, t239: F, t2456: F, t2501: F, t2698: F, t685: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39438, t39440, t39442, t39454, t39480, t39483) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2736::<F>(t10578, t9863, t762, t9291, t2629, t2, t588, t2576, t2565, t701);
        let (t39484, t39490, t39492, t39494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2737::<F>(t121, t4, t131, t268, t8779, t588, t9282, t239, t2456);
        let (t39495, t39497) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2738::<F>(t2501, t39494, t2698, t685);
    (t39438, t39440, t39442, t39454, t39480, t39483, t39484, t39490, t39492, t39494, t39495, t39497)
}
