//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta394(t3006: f64, t2986: f64, t973: f64, t981: f64, t11501: f64, t3011: f64, t4733: f64, t3014: f64, t1100: f64, t11108: f64, t12190: f64, t3329: f64, t3333: f64, t3336: f64, t41229: f64, t41241: f64, t41243: f64, t41449: f64, t41451: f64, t41453: f64, t41455: f64, t41459: f64, t5023: f64, t11506: f64, t41225: f64, t11610: f64, t3022: f64, t11396: f64, t3007: f64, t3033: f64, t11606: f64, t11571: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41464, t41468, t41472, t41476, t41477) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1449(t3006, t2986, t973, t981, t11501, t3011, t4733, t3014, t1100, t11108, t12190, t3329, t3333, t3336, t41229, t41241, t41243, t41449, t41451, t41453, t41455, t41459, t5023);
        let (t41481, t41483, t41485, t41488, t41490, t41491) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1450(t11506, t3014, t41225, t981, t11610, t3022, t11396, t3007, t3033, t11606, t11571, t300);
    (t41464, t41468, t41472, t41476, t41477, t41481, t41483, t41485, t41488, t41490, t41491)
}
