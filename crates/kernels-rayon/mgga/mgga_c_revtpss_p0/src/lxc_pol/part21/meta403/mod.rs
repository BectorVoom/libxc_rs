//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1865;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta403(t1224: f64, t3362: f64, t10356: f64, t1012: f64, t1226: f64, t697: f64, t1222: f64, t140: f64, t3688: f64, t3700: f64, t12268: f64, t3698: f64, t3367: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13007, t13008, t13011, t13012, t13014, t13015, t13017, t13018, t13020) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1865(t1224, t3362, t10356, t1012, t1226, t697, t1222, t140, t3688, t3700, t12268, t3698);
        let (t13021, t13022, t13026) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1866(t10356, t13020, t1012, t3367, t404);
    (t13007, t13008, t13011, t13012, t13014, t13015, t13017, t13018, t13021, t13022, t13026)
}
