//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2125/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125(t25082: f64, t75353: f64, t8717: f64, t7311: f64, t9593: f64, t28196: f64, t28198: f64, t28166: f64, t7234: f64, t28168: f64, t27153: f64, t32113: f64) -> (f64, f64, f64, f64) {
    let t98574 = 6.0_f64 * t25082 * t8717 * t75353;
    let t98575 = t7311 * t9593;
    let t98578 = 4.0_f64 * t28196 * t98575 * t28198;
    let t98579 = t7234 * t28166;
    let t98581 = 12.0_f64 * t98579 * t28168;
    let t98584 = 6.0_f64 * t25082 * t32113 * t27153;
    (t98574, t98578, t98581, t98584)
}
