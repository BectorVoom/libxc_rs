//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk218;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk219;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk220;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta28(t118: f64, t508: f64, t511: f64, t569: f64, param_d: f64, t116: f64, t117: f64, t10: f64, t2: f64, t17: f64, t16: f64, t3: f64, t15: f64, t14: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t571, t572) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk218(t118, t508, t511, t569, param_d);
        let t573 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk219(t116, t117);
        let (t575, t576, t578, t579, t580) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk220(t572, t573, t10, t2, t17, t16, t3);
        let (t582, t583) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk221(t15, t580, t14, t2);
    (t571, t572, t573, t575, t576, t578, t579, t580, t582, t583)
}
