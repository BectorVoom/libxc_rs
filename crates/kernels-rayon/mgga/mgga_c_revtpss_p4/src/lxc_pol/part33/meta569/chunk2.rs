//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1978/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1978(t4147: f64, t7311: f64, t1353: f64, t2033: f64, t7933: f64, t1518: f64, t2126: f64, t1450: f64, t11239: f64, t3736: f64, t211: f64, t9644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32113 = t4147 * t7311;
    let t32737 = t2033 * t1353;
    let t33651 = t4147 * t7933;
    let t34446 = t2126 * t1518;
    let t35669 = t7933 * t1450;
    let t37885 = t11239 * t3736;
    let t39643 = 1.0_f64 / t9644 / t211;
    (t32113, t32737, t33651, t34446, t35669, t37885, t39643)
}
