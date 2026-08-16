//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1929;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1930;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta377(t10389: f64, t1469: f64, t2299: f64, t4186: f64, t10398: f64, t2306: f64, t13312: f64, t2251: f64, t2258: f64, t4227: f64, t4232: f64, t606: f64, t633: f64, t637: f64, t77: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t13368, t13378, t13388) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1929(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1930(t13388, t77, t1469, t2258);
        let (t13393, t13396) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1931(t13392, t70, t4186, t606);
    (t13368, t13378, t13389, t13392, t13393, t13396)
}
