//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta584<F: Float>(t48269: F, t85912: F, t73481: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t47014: F, t47017: F, t47020: F, t47059: F, t73515: F, t74106: F, t48280: F, t48282: F, t48285: F, t48287: F, t48290: F, t47067: F, t47070: F, t47072: F, t47074: F, t47076: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91968, t91969, t91970, t91971) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1816::<F>(t48269, t85912, t73481, t39786, t39791, t39795, t39799, t39807, t39813, t47014, t47017, t47020, t47059);
        let (t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91981) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1817::<F>(t73515, t74106, t48280, t48282, t48285, t48287, t48290, t47067, t47070, t47072, t47074, t47076);
    (t91968, t91969, t91970, t91971, t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91981)
}
