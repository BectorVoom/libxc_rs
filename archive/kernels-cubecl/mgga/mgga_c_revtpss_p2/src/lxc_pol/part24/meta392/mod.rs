//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1303;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1304;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1305;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1306;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta392<F: Float>(t128: F, t121: F, t268: F, t8779: F, t588: F, t9295: F, t2508: F, t39494: F, t39497: F, t692: F, t124: F, t138: F, t239: F, t39490: F, t39492: F, t39495: F, t39498: F, t39501: F, t682: F, t701: F, t2566: F, t9274: F, t2584: F, t9311: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39506, t39508, t39510, t39512, t39515) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1303::<F>(t128, t121, t268, t8779, t588, t9295, t2508, t39494, t39497, t692, t124, t138, t239);
        let t39520 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1304::<F>(t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515, t682, t701);
        let (t39525, t39528) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1305::<F>(t2566, t701, t9274);
        let t39531 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1306::<F>(t2584, t39525, t9311);
    (t39506, t39508, t39510, t39512, t39515, t39520, t39525, t39528, t39531)
}
