//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2652;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta675(t20800: f64, t5465: f64, t5480: f64, t3302: f64, t471: f64, t1214: f64, t20795: f64, t1287: f64, t21298: f64, t5464: f64, t21164: f64, t20900: f64, t487: f64, t1770: f64, t5462: f64, t12050: f64, t1248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21465, t21468, t21471) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2652(t20800, t5465, t5480, t3302, t471);
        let (t21473, t21480, t21484, t21491, t21495, t21500, t21506) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2653(t1214, t21471, t20795, t1287, t21298, t5464, t21164, t20900, t487, t1770, t5462, t12050, t1248, t471);
    (t21465, t21468, t21471, t21473, t21480, t21484, t21491, t21495, t21500, t21506)
}
