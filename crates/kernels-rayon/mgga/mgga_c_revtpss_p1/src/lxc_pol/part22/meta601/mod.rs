//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta601(t11387: f64, t6109: f64, t934: f64, t11385: f64, t6158: f64, t953: f64, t1622: f64, t4669: f64, t6177: f64, t6174: f64, t2970: f64, t6173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19255, t19256, t19258, t19263, t19266, t19269, t19272, t19275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2491(t11387, t6109, t934, t11385, t6158, t953, t1622, t4669, t6177, t6174, t2970, t6173);
    (t19255, t19256, t19258, t19263, t19266, t19269, t19272, t19275)
}
