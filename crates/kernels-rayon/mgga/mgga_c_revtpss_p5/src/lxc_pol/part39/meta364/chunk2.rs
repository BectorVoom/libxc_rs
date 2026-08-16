//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1273/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273(t15220: f64, t923: f64, t916: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11339: f64, t11366: f64, t11368: f64, t11479: f64, t11480: f64) -> (f64, f64, f64) {
    let t15221 = t923 * t15220;
    let t15230 = t916 * t15220;
    let t15232 = -t11479 - t11480 + 0.16504875e0_f64 * t15221 + 0.18396666666666666667e-1_f64 * t11339 - 0.20128333333333333334e0_f64 * t11138 - 0.26837777777777777778e0_f64 * t11134 + 0.10064166666666666667e0_f64 * t11140 + 0.67094444444444444447e-1_f64 * t11136 - 0.18396666666666666667e0_f64 * t11366 + 0.5519e-1_f64 * t11368 + 0.258925e1_f64 * t15230;
    (t15221, t15230, t15232)
}
