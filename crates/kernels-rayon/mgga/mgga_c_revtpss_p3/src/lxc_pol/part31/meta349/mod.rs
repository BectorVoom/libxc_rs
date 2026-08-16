//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta349(t221: f64, t5627: f64, t9921: f64, t3978: f64, t2619: f64, t5635: f64, t1398: f64, t1882: f64, t13848: f64, t3938: f64, t9818: f64, t9816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13877, t13878, t13880, t13887, t13926, t13941, t13943) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1362(t221, t5627, t9921, t3978, t2619, t5635, t1398, t1882, t13848, t3938, t9818, t9816);
    (t13877, t13878, t13880, t13887, t13926, t13941, t13943)
}
