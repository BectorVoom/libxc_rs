//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk857;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk858;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk859;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta158(t1065: f64, t905: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1043: f64, t3154: f64, t3317: f64, t357: f64, t999: f64, t1012: f64, t1014: f64, t3252: f64, t354: f64, t3298: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4872, t4890, t4891) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk857(t1065, t905, t3147, t72, t3088);
        let t4892 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk858(t3299, t4891);
        let (t4894, t4899) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk859(t1043, t3154, t3317, t4891);
        let (t4900, t4910, t4915, t4919, t4976, t4980) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk860(t1043, t357, t999, t1012, t1014, t3252, t354, t3298, t378);
    (t4872, t4890, t4891, t4892, t4894, t4899, t4900, t4910, t4915, t4919, t4976, t4980)
}
