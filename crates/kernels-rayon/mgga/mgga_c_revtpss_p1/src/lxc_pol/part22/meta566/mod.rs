//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta566(t5883: f64, t648: f64, t1501: f64, t670: f64, t6765: f64, t1843: f64, t4292: f64, t1310: f64, t5920: f64, t116: f64, t5876: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t18220, t18227, t18232, t18235, t18242, t18245) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2409(t5883, t648, t1501, t670, t6765, t1843, t4292, t1310, t5920, t116, t5876);
    (t18220, t18227, t18232, t18235, t18242, t18245)
}
