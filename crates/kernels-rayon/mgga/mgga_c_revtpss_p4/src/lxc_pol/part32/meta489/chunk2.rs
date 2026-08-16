//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1744/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744(t25431: f64, t28368: f64, t25411: f64, t786: f64, t7998: f64, t789: f64, t231: f64, t7997: f64, t836: f64, t7076: f64, t1558: f64, t7398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28369 = t25431 * t28368;
    let t28371 = t25411 * t28368;
    let t28373 = t786 * t7998;
    let t28374 = t28373 * t789;
    let t28377 = t7997 * t836 * t231;
    let t28378 = t7076 * t28377;
    let t28384 = t7398 * t1558 * t231;
    (t28369, t28371, t28373, t28374, t28377, t28378, t28384)
}
