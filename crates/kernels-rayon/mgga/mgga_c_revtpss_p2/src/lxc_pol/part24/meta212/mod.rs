//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk954;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk955;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta212(t992: f64, t338: f64, t378: f64, t1031: f64, t342: f64, t3145: f64, t334: f64, t368: f64, t365: f64, t3144: f64, t3153: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk954(t992, t338);
        let (t11201, t11238, t11239) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk955(t11200, t378, t1031);
        let (t11240, t11243, t11244, t11245, t11246, t11249) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk956(t11239, t342, t3145, t334, t368, t365, t3144, t3153, t73);
    (t11198, t11199, t11200, t11201, t11238, t11239, t11240, t11243, t11244, t11245, t11246, t11249)
}
