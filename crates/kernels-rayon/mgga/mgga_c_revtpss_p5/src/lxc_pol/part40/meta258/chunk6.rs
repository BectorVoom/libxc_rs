//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 969/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk969(t1312: f64, t2199: f64, t2201: f64, t2322: f64, t4254: f64, t5523: f64, t651: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64, t3: f64) -> (f64, f64) {
    let t8330 = 2.0_f64 * t1312 * t8325 + 2.0_f64 * t1312 * t8327 - 2.0_f64 * t2199 * t2322 - 2.0_f64 * t2199 * t4254 + 2.0_f64 * t2201 * t2322 + 2.0_f64 * t2201 * t5523 - 2.0_f64 * t651 * t8307 - 2.0_f64 * t651 * t8321;
    let t8331 = t3 * t8330;
    (t8330, t8331)
}
