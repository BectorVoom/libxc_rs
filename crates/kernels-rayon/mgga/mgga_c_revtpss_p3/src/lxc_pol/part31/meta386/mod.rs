//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta386(t3172: f64, t4868: f64, t1041: f64, t3168: f64, t4878: f64, t11150: f64, t3181: f64, t11144: f64, t11852: f64, t3124: f64, t4820: f64, t1655: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16163, t16165, t16190, t16199, t16208, t16218, t16219) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1424(t3172, t4868, t1041, t3168, t4878, t11150, t3181, t11144, t11852, t3124, t4820, t1655, t697);
    (t16163, t16165, t16190, t16199, t16208, t16218, t16219)
}
