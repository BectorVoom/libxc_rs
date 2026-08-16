//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1504/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1504(t1913: f64, t8302: f64, t2192: f64, t5789: f64, t116890: f64, t117095: f64, t117369: f64, t117374: f64, t117720: f64, t117765: f64, t1458: f64, t1464: f64, t18178: f64, t1921: f64, t31088: f64, t31329: f64, t4154: f64, t4168: f64, t5790: f64, t8373: f64, t8389: f64) -> f64 {
    let t117772 = 2.0_f64 * t1913 * t8302;
    let t117774 = 2.0_f64 * t5789 * t2192;
    let t117777 = t117095 + t117369 + 2.0_f64 * t5790 * t8302 + t4154 * t8389 + t117374 + t1458 * (t117720 + t117765) + t116890 + t18178 * t2192 + t8373 * t4168 + t31088 * t1921 + t117772 + t117774 + 2.0_f64 * t31329 * t1464;
    t117777
}
