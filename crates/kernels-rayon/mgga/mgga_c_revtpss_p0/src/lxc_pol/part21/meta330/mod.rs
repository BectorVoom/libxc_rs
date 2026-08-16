//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1636;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta330(t2258: f64, t606: f64, t4801: f64, t1042: f64, t1031: f64, t342: f64, t3145: f64, t334: f64, t368: f64, t365: f64, t3144: f64, t1043: f64, t3151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t11231 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1635(t2258, t606);
        let (t11232, t11233, t11238, t11239) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1636(t11231, t4801, t1042, t1031);
        let (t11240, t11243, t11244, t11245, t11246, t11247) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1637(t11239, t342, t3145, t334, t368, t365, t3144, t1043, t3151);
    (t11231, t11232, t11233, t11238, t11239, t11240, t11243, t11244, t11245, t11246, t11247)
}
