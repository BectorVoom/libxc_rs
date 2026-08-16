//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta539(t29524: f64, t72: f64, t1927: f64, t7715: f64, t7719: f64, t5868: f64, t76: f64, t1926: f64, t1470: f64, t4173: f64, t1493: f64, t1497: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29525, t29526, t29529, t29532, t29533, t29538, t29543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1924(t29524, t72, t1927, t7715, t7719, t5868, t76, t1926, t1470, t4173, t1493, t1497);
    (t29525, t29526, t29529, t29532, t29533, t29538, t29543)
}
