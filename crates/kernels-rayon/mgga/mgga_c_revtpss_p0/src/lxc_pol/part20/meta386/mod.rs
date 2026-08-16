//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta386(t11341: f64, t141: f64, t41248: f64, t10326: f64, t2857: f64, t606: f64, t930: f64, t2852: f64, t2908: f64, t11150: f64, t2251: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t41250, t41253, t41255, t41258, t41260, t41263) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1416(t11341, t141, t41248, t10326, t2857, t606, t930, t2852, t2908, t11150, t2251, t2258);
    (t41250, t41253, t41255, t41258, t41260, t41263)
}
