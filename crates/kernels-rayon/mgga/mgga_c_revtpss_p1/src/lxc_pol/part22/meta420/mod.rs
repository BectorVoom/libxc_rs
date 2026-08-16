//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta420(t45: f64, t11064: f64, t1583: f64, t1469: f64, t2609: f64, t706: f64, t10593: f64, t10597: f64, t4186: f64, t80: f64, t13312: f64, t1490: f64, t2251: f64, t2258: f64, t4328: f64, t606: f64, t766: f64, zeta_threshold: f64, t57: f64, t83: f64, t1491: f64, t4335: f64, t770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14436, t14440, t14441, t14442, t14443, t14444, t14455) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2029(t45, t11064, t1583, t1469, t2609, t706, t10593, t10597, t4186, t80, t13312, t1490, t2251, t2258, t4328, t606, t766, zeta_threshold);
        let t14468 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2030(t57, t4186, t83, t13312, t1491, t2251, t2258, t4335, t606, t770, t14455, zeta_threshold);
    (t14436, t14440, t14441, t14442, t14443, t14444, t14468)
}
