//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta316(t1466: f64, t2246: f64, t2275: f64, t4186: f64, t580: f64, t9342: f64, t2282: f64, t10389: f64, t1469: f64, t2299: f64, t10398: f64, t2306: f64, t116: f64, t4245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604(t1466, t2246, t2275, t4186, t580, t9342, t2282, t10389, t1469, t2299, t10398, t2306);
        let t13426 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1605(t116, t4245);
    (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381, t13426)
}
