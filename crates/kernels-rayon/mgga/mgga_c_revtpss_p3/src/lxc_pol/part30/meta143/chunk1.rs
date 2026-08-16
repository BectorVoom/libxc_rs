//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 770/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk770(t1045: f64, t3133: f64, t373: f64, t1042: f64, t1031: f64, t196: f64) -> (f64, f64, f64) {
    let t3135 = t373 * t3133 * t1045;
    let t3136 = t1042 * t3135;
    let t3140 = 1.0_f64 / t1031 / t196;
    (t3135, t3136, t3140)
}
