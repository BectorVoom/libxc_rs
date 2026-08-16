//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta178(t555: f64, t9646: f64, t1358: f64, t22: f64, t1444: f64, t4131: f64, t4076: f64, t1425: f64, t225: f64, t4077: f64, t3907: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9647, t9648, t9650, t9651, t9652, t9655, t9656, t9657, t9658, t9659, t9664) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk920(t555, t9646, t1358, t22, t1444, t4131, t4076, t1425, t225, t4077, t3907, t9285);
    (t9647, t9648, t9650, t9651, t9652, t9655, t9656, t9657, t9658, t9659, t9664)
}
