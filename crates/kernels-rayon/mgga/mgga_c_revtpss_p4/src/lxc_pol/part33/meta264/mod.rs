//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta264(t532: f64, t7311: f64, t1450: f64, t2014: f64, t1448: f64, t4147: f64, t2034: f64, t1459: f64, t2042: f64, t116: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7312, t7313, t7314, t7315, t7316, t7317, t7329, t7330) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1179(t532, t7311, t1450, t2014, t1448, t4147, t2034, t1459, t2042, t116, t1936);
    (t7312, t7313, t7314, t7315, t7316, t7317, t7329, t7330)
}
