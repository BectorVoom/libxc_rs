//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta504(t2661: f64, t2662: f64, t4352: f64, t6017: f64, t23285: f64, t2741: f64, t23289: f64, t6035: f64, t61625: f64, t23342: f64, t2652: f64, t221: f64, t23114: f64, t2674: f64, t40683: f64, t14648: f64, t14832: f64, t5962: f64, t23346: f64, t231: f64, t76569: f64, t23244: f64, t243: f64, t10871: f64, t40693: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76764, t76767, t76793, t76797, t76804, t76808) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511(t2661, t2662, t4352, t6017, t23285, t2741, t23289, t6035, t61625, t23342, t2652, t221, t23114, t2674, t40683);
        let (t76812, t76814, t76818, t76823, t76827) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1512(t14648, t14832, t2661, t5962, t23346, t2652, t231, t2662, t76569, t23244, t243, t10871, t40693);
    (t76764, t76767, t76793, t76797, t76804, t76808, t76812, t76814, t76818, t76823, t76827)
}
