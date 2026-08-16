//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 738/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk738(t3: f64, t3931: f64, t112: f64, t1395: f64, t111: f64, t576: f64, t1401: f64, t2319: f64, t2363: f64, t577: f64, t671: f64, t2218: f64, t2221: f64, t2225: f64, t2232: f64) -> (f64, f64, f64, f64, f64) {
    let t3932 = t3 * t3931;
    let t3938 = t1395 * t112;
    let t3941 = t576 * t111;
    let t3946 = 0.45e1_f64 * t3931 * t577 + 27.0_f64 * t3938 * t671 + 27.0_f64 * t3941 * t2319 + 0.135e2_f64 * t1401 * t2363;
    let t3951 = -t2218 - 0.78e0_f64 * t2221 - 0.578e2_f64 * t2225 + t2232;
    (t3932, t3938, t3941, t3946, t3951)
}
