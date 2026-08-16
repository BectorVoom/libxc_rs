//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta410(t4292: f64, t648: f64, t13514: f64, t94: f64, t1513: f64, t2340: f64, t4287: f64, t665: f64, t2366: f64, t93: f64, t31087: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t98487, t98535, t101457, t101460, t101463, t101522, t116890) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1487(t4292, t648, t13514, t94, t1513, t2340, t4287, t665, t2366, t93, t31087, t575);
    (t98487, t98535, t101457, t101460, t101463, t101522, t116890)
}
