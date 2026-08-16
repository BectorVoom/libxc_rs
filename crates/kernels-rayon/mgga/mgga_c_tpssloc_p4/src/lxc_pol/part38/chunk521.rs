//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 521/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk521(t2221: f64, t587: f64, t591: f64, t14: f64, t21: f64) -> (f64, f64, f64, f64) {
    let t2222 = 0.1122e2_f64 * t2221;
    let t2223 = t587 * t591;
    let t2224 = 16.0_f64 * t2223;
    let t2225 = t14 * t21;
    (t2222, t2223, t2224, t2225)
}
