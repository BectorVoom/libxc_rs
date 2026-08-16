//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1078;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1079;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1080;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta248(t11231: f64, t4801: f64, t1042: f64, t1031: f64, t342: f64, t3145: f64, t334: f64, t368: f64, t365: f64, t3144: f64, t1043: f64, t3151: f64, t373: f64, t3153: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11232, t11233, t11238, t11239) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1078(t11231, t4801, t1042, t1031);
        let t11240 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1079(t11239, t342);
        let (t11243, t11244, t11245, t11246, t11247) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1080(t3145, t334, t368, t365, t3144, t11240, t1043, t3151);
        let (t11248, t11249) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1081(t11247, t373, t3153, t73);
    (t11232, t11233, t11238, t11239, t11240, t11243, t11244, t11245, t11246, t11247, t11248, t11249)
}
