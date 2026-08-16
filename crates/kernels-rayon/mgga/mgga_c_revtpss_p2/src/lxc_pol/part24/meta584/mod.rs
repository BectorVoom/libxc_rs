//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta584(t48269: f64, t85912: f64, t73481: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t47014: f64, t47017: f64, t47020: f64, t47059: f64, t73515: f64, t74106: f64, t48280: f64, t48282: f64, t48285: f64, t48287: f64, t48290: f64, t47067: f64, t47070: f64, t47072: f64, t47074: f64, t47076: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91968, t91969, t91970, t91971) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1816(t48269, t85912, t73481, t39786, t39791, t39795, t39799, t39807, t39813, t47014, t47017, t47020, t47059);
        let (t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91981) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1817(t73515, t74106, t48280, t48282, t48285, t48287, t48290, t47067, t47070, t47072, t47074, t47076);
    (t91968, t91969, t91970, t91971, t91974, t91975, t91976, t91977, t91978, t91979, t91980, t91981)
}
