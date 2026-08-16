//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta635(t16060: f64, t7111: f64, t25539: f64, t4924: f64, t16219: f64, t139: f64, t27526: f64, t3252: f64, t4574: f64, t1014: f64, t4579: f64, t1035: f64, t27543: f64, t7150: f64, t99708: f64, t1977: f64, t994: f64, t11627: f64, t1983: f64, t99682: f64, t11223: f64, t7143: f64, t3057: f64, t7810: f64, t11120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100359, t100363, t100365, t100370, t100398, t100431) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2089(t16060, t7111, t25539, t4924, t16219, t139, t27526, t3252, t4574, t1014, t4579, t1035, t27543);
        let (t100494, t100586, t100596, t100658, t100681, t100690) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090(t7150, t99708, t1977, t994, t11627, t1983, t99682, t11223, t7143, t3057, t7810, t11120);
    (t100359, t100363, t100365, t100370, t100398, t100431, t100494, t100586, t100596, t100658, t100681, t100690)
}
