//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta283(t1052: f64, t3147: f64, t1036: f64, t3141: f64, t3229: f64, t369: f64, t361: f64, t351: f64, t3106: f64, t3111: f64, t3156: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11997, t11998, t11999, t12003, t12004, t12007, t12009) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1145(t1052, t3147, t1036, t3141, t3229, t369, t361, t351, t3106, t3111, t3156, t3172);
    (t11997, t11998, t11999, t12003, t12004, t12007, t12009)
}
