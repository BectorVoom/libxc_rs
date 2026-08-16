//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta804 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta804(t1340: f64, t40165: f64, t2626: f64, t9551: f64, t268: f64, t520: f64, t39768: f64, t190: f64, t22: f64, t519: f64, t39762: f64, t40129: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t47059, t47060, t47067, t47070, t47072, t47076) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2906(t1340, t40165, t2626, t9551, t268, t520, t39768, t190, t22, t519, t39762, t40129);
    (t47059, t47060, t47067, t47070, t47072, t47076)
}
