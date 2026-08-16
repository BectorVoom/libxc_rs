//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta937 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta937(t12808: f64, t21013: f64, t1222: f64, t3698: f64, t5047: f64, t697: f64, t12855: f64, t12916: f64, t17455: f64, t16738: f64, t17240: f64, t16742: f64, t16733: f64, t12772: f64, t17678: f64, t5340: f64, t17683: f64, t5331: f64, t12832: f64, t17620: f64, t17412: f64, t3636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57710, t57726, t57735, t57743, t57746) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3171(t12808, t21013, t1222, t3698, t5047, t697, t12855, t12916, t17455, t16738, t17240, t16742);
        let (t57749, t57770, t57773, t57780, t57786) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3172(t1222, t16733, t17240, t12772, t17678, t5340, t17683, t5331, t12832, t17620, t17412, t3636);
    (t57710, t57726, t57735, t57743, t57746, t57749, t57770, t57773, t57780, t57786)
}
