//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2458;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta663(t11875: f64, t11876: f64, t11922: f64, t11991: f64, t3111: f64, t1062: f64, t11903: f64, t11988: f64, t3188: f64, t11263: f64, t3124: f64, t11262: f64, t3150: f64, t3156: f64, t3161: f64, t3163: f64, t11267: f64, t3123: f64, t12016: f64, t3115: f64, t11638: f64, t3127: f64, t3172: f64, t11683: f64, t11710: f64, t3091: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42900, t42902, t42904, t42907, t42926, t42929) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2458(t11875, t11876, t11922, t11991, t3111, t1062, t11903, t11988, t3188, t11263, t3124, t11262, t3150, t3156);
        let (t42932, t42934, t42947, t42962, t42965) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2459(t11262, t3161, t3163, t11267, t3123, t11922, t12016, t3115, t11638, t3127, t3172, t11683, t11710, t3091);
    (t42900, t42902, t42904, t42907, t42926, t42929, t42932, t42934, t42947, t42962, t42965)
}
