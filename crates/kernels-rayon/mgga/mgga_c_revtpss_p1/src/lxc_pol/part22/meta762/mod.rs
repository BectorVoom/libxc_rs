//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta762 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta762(t1086: f64, t11200: f64, t3090: f64, t11671: f64, t11926: f64, t16565: f64, t994: f64, t42859: f64, t42862: f64, t342: f64, t3145: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t43291, t43297, t43341, t43346, t43347, t43350) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2844(t1086, t11200, t3090, t11671, t11926, t16565, t994, t42859, t42862, t342, t3145, t368);
    (t43291, t43297, t43341, t43346, t43347, t43350)
}
