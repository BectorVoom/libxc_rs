//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1300;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1301;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1302;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta391<F: Float>(t121: F, t4: F, t131: F, t268: F, t8779: F, t588: F, t9282: F, t239: F, t2456: F, t2501: F, t2698: F, t685: F, t684: F, t125: F, t123: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39484, t39490, t39492, t39494) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1300::<F>(t121, t4, t131, t268, t8779, t588, t9282, t239, t2456);
        let (t39495, t39497) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1301::<F>(t2501, t39494, t2698, t685);
        let (t39498, t39500, t39501) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1302::<F>(t39497, t684, t125, t2698, t123);
    (t39484, t39490, t39492, t39494, t39495, t39497, t39498, t39500, t39501)
}
