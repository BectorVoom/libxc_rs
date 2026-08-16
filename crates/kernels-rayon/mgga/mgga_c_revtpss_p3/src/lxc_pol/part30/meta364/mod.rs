//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta364(t3618: f64, t828: f64, t1260: f64, t3650: f64, t3588: f64, t73: f64, t1209: f64, t3781: f64, t5330: f64, t3153: f64, t3601: f64, t1284: f64, t3555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12787, t12800, t12803, t12808, t12809, t12810, t12831) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1390(t3618, t828, t1260, t3650, t3588, t73, t1209, t3781, t5330, t3153, t3601, t1284, t3555);
    (t12787, t12800, t12803, t12808, t12809, t12810, t12831)
}
