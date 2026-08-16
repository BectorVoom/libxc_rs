//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2013/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013(t10948: f64, t23540: f64, t10472: f64, t10478: f64, t6753: f64, t10375: f64, t1942: f64, t23488: f64, t23509: f64, t23508: f64, t6721: f64, t6741: f64) -> (f64, f64, f64, f64, f64) {
    let t83061 = t10948 * t23540;
    let t83065 = t10472 * t6753 * t10478;
    let t83080 = t1942 * t10375 / 5184.0_f64;
    let t83117 = t23509 * t23488;
    let t83120 = t6721 * t23508;
    let t83121 = t83120 * t6741;
    (t83061, t83065, t83080, t83117, t83121)
}
