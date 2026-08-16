//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk985;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta209(t30: f64, t33: f64, t1927: f64, t2258: f64, t2251: f64, t627: f64, t9344: f64, zeta_threshold: f64, t36: f64, t70: f64, t2259: f64, t2291: f64, t607: f64, t363: f64, t41: f64, t46: f64, t47: f64, t606: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10318, t10321, t10326) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk985(t30, t33, t1927, t2258, t2251, t627, t9344, zeta_threshold);
        let (t10327, t10328, t10331, t10336, t10344, t10345, t10355, t10356) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk986(t10326, t36, t70, t2259, t627, t2291, t607, t363, t41, t46, t47, t2251, t606, sigma0);
    (t10318, t10321, t10326, t10327, t10328, t10331, t10336, t10344, t10345, t10355, t10356)
}
