//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta313(t10290: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64, t580: f64, t9342: f64, t116: f64, t4245: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t13266, t13269, t13272, t13309, t13310, t13426) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1084(t10290, t4171, t602, t1466, t2246, t580, t9342, t116, t4245);
    (t13266, t13269, t13272, t13309, t13310, t13426)
}
