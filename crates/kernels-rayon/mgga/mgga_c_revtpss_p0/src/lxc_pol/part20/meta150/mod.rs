//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk820;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk821;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk822;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk823;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta150(t239: f64, t4000: f64, t820: f64, t543: f64, t3923: f64, t1390: f64, t828: f64, t531: f64, t549: f64, t240: f64, t72: f64, t3829: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk820(t239, t4000, t820, t543);
        let t4004 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk821(t3923, t4003);
        let (t4006, t4010) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk822(t1390, t4004, t828, t531, t549);
        let (t4011, t4012, t4014, t4018) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk823(t240, t4010, t72, t3829, t828, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk824(t136, t1389);
    (t4002, t4003, t4004, t4006, t4010, t4011, t4012, t4014, t4018, t4019)
}
