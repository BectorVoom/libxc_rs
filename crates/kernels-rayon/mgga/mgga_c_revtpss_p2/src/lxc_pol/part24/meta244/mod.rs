//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta244(t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64, t2798: f64, t1568: f64, t2783: f64, t786: f64, t2435: f64, t4519: f64, t1558: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14557, t14558, t14563, t14564, t14567, t14568, t14581, t14586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1006(t2777, t4518, t2439, t2470, t4499, t2798, t1568, t2783, t786, t2435, t4519, t1558, t2723);
    (t14557, t14558, t14563, t14564, t14567, t14568, t14581, t14586)
}
