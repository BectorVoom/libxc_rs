//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1564;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1565;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta306(t240: f64, t2719: f64, t243: f64, t2722: f64, t2723: f64, t2661: f64, t231: f64, t2662: f64, t10489: f64, t828: f64, t855: f64, t221: f64, t2430: f64, t2675: f64, t2674: f64, t2735: f64, t2783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10726 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1564(t240, t2719);
        let (t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1565(t243, t2722, t2723, t10726, t2661, t231, t2662, t10489, t828, t855, t221, t2430, t2675);
        let (t10742, t10744) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1566(t10741, t2674, t2735, t2783);
    (t10726, t10728, t10729, t10730, t10732, t10733, t10734, t10737, t10741, t10742, t10744)
}
