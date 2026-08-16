//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta786(t10: f64, t22: f64, t15: f64, t27: f64, t11: f64, t2224: f64, t20: f64, t2237: f64, t12: f64, t14: f64, t2231: f64, t25: f64, t40649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45926, t45931, t45933, t45936, t45941, t45944, t45947, t45952) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2876(t10, t22, t15, t27, t11, t2224, t20, t2237, t12, t14, t2231, t25, t40649);
    (t45926, t45931, t45933, t45936, t45941, t45944, t45947, t45952)
}
