//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2107;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta655(t29490: f64, t571: f64, t1459: f64, t30188: f64, t116: f64, t30004: f64, t572: f64, t670: f64, t1518: f64, t1936: f64, t4292: f64, t6941: f64, t7334: f64, t30194: f64, t21881: f64, t7330: f64, t1916: f64, t28271: f64, t28268: f64, t30185: f64, t25082: f64, t86771: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105804, t105818, t105822, t105826, t105830) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2107(t29490, t571, t1459, t30188, t116, t30004, t572, t670, t1518, t1936, t4292, t6941, t7334);
        let (t105834, t105837, t105839, t105841, t105843, t105859) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2108(t1459, t30194, t21881, t572, t7330, t1916, t28271, t28268, t30185, t25082, t86771, t8717);
    (t105804, t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t105859)
}
