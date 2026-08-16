//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk645;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk646;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta103<F: Float>(t2496: F, t760: F, t128: F, t131: F, t136: F, t2457: F, t2470: F, t684: F, t692: F, t2435: F, t2439: F, t738: F, t745: F, t675: F, t681: F, t268: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2498, t2501, t2502, t2504, t2508, t2509, t2511, t2514) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk645::<F>(t2496, t760, t128, t131, t136, t2457, t2470, t684, t692, t2435, t2439);
        let t2516 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk646::<F>(t2514, t738, t745);
        let (t2518, t2519, t2522) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk647::<F>(t2516, t760, t675, t681, t268, t702);
    (t2498, t2501, t2502, t2504, t2508, t2509, t2511, t2514, t2516, t2518, t2519, t2522)
}
