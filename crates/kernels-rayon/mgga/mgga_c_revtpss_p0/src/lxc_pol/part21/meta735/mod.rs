//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2584;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta735(t3906: f64, t3907: f64, t39494: f64, t1426: f64, t4067: f64, t786: f64, t3917: f64, t2453: f64, t3908: f64, t10115: f64, t1421: f64, t10168: f64, t3920: f64, t10174: f64, t9676: f64, t123: f64, t2434: f64, t3915: f64, t4131: f64, t10175: f64, t9686: f64, t1420: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47504, t47506, t47507, t47510, t47512, t47516) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2584(t3906, t3907, t39494, t1426, t4067, t786, t3917, t2453, t3908, t10115, t1421, t10168, t3920);
        let (t47520, t47521, t47525, t47527, t47530) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585(t10174, t2453, t9676, t123, t2434, t3915, t4131, t10175, t9686, t1420, t4075, t786);
    (t47504, t47506, t47507, t47510, t47512, t47516, t47520, t47521, t47525, t47527, t47530)
}
