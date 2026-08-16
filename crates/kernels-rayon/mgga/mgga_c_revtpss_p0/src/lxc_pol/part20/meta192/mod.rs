//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta192(t4056: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t240: f64, t4000: f64, t4003: f64, t9768: f64, t532: f64, t549: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9929, t9930, t9931, t9932, t9934, t9935, t9936, t9937, t9940, t9941, t9942) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk952(t4056, t550, t543, t3992, t2661, t240, t4000, t4003, t9768, t532, t549, t72);
    (t9929, t9930, t9931, t9932, t9934, t9935, t9936, t9937, t9940, t9941, t9942)
}
