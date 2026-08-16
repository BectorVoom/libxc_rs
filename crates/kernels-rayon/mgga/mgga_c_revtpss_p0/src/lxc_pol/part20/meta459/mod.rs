//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1749;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta459(t1317: f64, t9561: f64, t1340: f64, t40182: f64, t39821: f64, t40196: f64, t9554: f64, t40192: f64, t4038: f64, t9419: f64, t40113: f64, t40169: f64, t2516: f64, t9551: f64, t3863: f64, t4029: f64, t39989: f64, t40067: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1749(t1317, t9561, t1340, t40182, t39821, t40196, t9554, t40192, t4038, t9419, t40113, t40169);
        let (t47100, t47102, t47103) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1750(t2516, t9551, t3863, t4029, t39989, t40067, t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098);
    (t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098, t47100, t47102, t47103)
}
