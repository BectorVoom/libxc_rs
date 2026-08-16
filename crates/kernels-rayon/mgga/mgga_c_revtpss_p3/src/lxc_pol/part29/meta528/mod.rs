//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1856;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta528(t26703: f64, t575: f64, t26743: f64, t571: f64, t1455: f64, t7560: f64, t2110: f64, t4168: f64, t1923: f64, t25146: f64, t7348: f64, t25150: f64, t7349: f64, t26169: f64, t6954: f64, t26204: f64, t6977: f64, t25117: f64, t1927: f64, t72: f64, t843: f64, t26205: f64, t45958: f64, t7342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95184, t95186, t95190, t95196, t95230, t95241) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1856(t26703, t575, t26743, t571, t1455, t7560, t2110, t4168, t1923, t25146, t7348, t25150, t7349);
        let (t95243, t95246, t95248, t95253, t95255, t95259) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857(t26169, t6954, t1923, t26204, t6977, t25117, t7349, t1927, t72, t843, t26205, t45958, t7342);
    (t95184, t95186, t95190, t95196, t95230, t95241, t95243, t95246, t95248, t95253, t95255, t95259)
}
