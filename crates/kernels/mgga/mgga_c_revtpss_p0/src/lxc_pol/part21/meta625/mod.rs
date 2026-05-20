//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2386;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta625<F: Float>(t10292: F, t65: F, t235: F, t2710: F, t826: F, t225: F, t785: F, t2737: F, t2694: F, t9789: F, t853: F, t9794: F, t775: F, t837: F, t10760: F, t66: F, t240: F, t10688: F, t243: F, t268: F, t9784: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40604, t40607, t40609, t40611, t40625, t40627) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2386::<F>(t10292, t65, t235, t2710, t826, t225, t785, t2737, t2694, t9789, t853, t9794);
        let (t40628, t40630, t40633, t40634, t40638, t40639) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2387::<F>(t775, t837, t10760, t40627, t10292, t66, t240, t10688, t243, t268, t2694, t9784);
    (t40604, t40607, t40609, t40611, t40625, t40627, t40628, t40630, t40633, t40634, t40638, t40639)
}
