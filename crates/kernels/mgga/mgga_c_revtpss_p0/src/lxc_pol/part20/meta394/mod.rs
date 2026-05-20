//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta394<F: Float>(t3006: F, t2986: F, t973: F, t981: F, t11501: F, t3011: F, t4733: F, t3014: F, t1100: F, t11108: F, t12190: F, t3329: F, t3333: F, t3336: F, t41229: F, t41241: F, t41243: F, t41449: F, t41451: F, t41453: F, t41455: F, t41459: F, t5023: F, t11506: F, t41225: F, t11610: F, t3022: F, t11396: F, t3007: F, t3033: F, t11606: F, t11571: F, t300: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t41464, t41468, t41472, t41476, t41477) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449::<F>(t3006, t2986, t973, t981, t11501, t3011, t4733, t3014, t1100, t11108, t12190, t3329, t3333, t3336, t41229, t41241, t41243, t41449, t41451, t41453, t41455, t41459, t5023);
        let (t41481, t41483, t41485, t41488, t41490, t41491) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450::<F>(t11506, t3014, t41225, t981, t11610, t3022, t11396, t3007, t3033, t11606, t11571, t300);
    (t41464, t41468, t41472, t41476, t41477, t41481, t41483, t41485, t41488, t41490, t41491)
}
