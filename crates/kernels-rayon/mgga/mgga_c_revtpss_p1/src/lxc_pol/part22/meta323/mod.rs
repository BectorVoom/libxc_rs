//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta323(t243: f64, t2722: f64, t2723: f64, t10726: f64, t2661: f64, t231: f64, t2662: f64, t221: f64, t2430: f64, t2675: f64, t2674: f64, t2735: f64, t2783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10728, t10729, t10730, t10732, t10733, t10734, t10741, t10742, t10744) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1769(t243, t2722, t2723, t10726, t2661, t231, t2662, t221, t2430, t2675, t2674, t2735, t2783);
    (t10728, t10729, t10730, t10732, t10733, t10734, t10741, t10742, t10744)
}
