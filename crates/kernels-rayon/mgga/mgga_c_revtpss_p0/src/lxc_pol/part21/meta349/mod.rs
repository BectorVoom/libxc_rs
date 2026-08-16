//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta349(t3133: f64, t73: f64, t3095: f64, t3092: f64, t2858: f64, t4786: f64, t3153: f64, t4894: f64, t3117: f64, t4900: f64, t2258: f64, t3094: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11678, t11679, t11680, t11683, t11684, t11687, t11688, t11689, t11692, t11693, t11696) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1687(t3133, t73, t3095, t3092, t2858, t4786, t3153, t4894, t3117, t4900, t2258, t3094);
    (t11678, t11679, t11680, t11683, t11684, t11687, t11688, t11689, t11692, t11693, t11696)
}
