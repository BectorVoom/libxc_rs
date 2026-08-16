//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1503/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1503(t116: f64, t31292: f64, t117: f64, t117103: f64, t117575: f64, t13514: f64, t1459: f64, t1461: f64, t1518: f64, t1916: f64, t2327: f64, t2371: f64, t31114: f64, t31117: f64, t31124: f64, t31340: f64, t31359: f64, t31362: f64, t31365: f64, t31370: f64, t31371: f64, t31374: f64, t4158: f64, t4292: f64, t572: f64, t5802: f64, t670: f64, t8289: f64, t8295: f64, t8362: f64, t8383: f64, t8386: f64) -> f64 {
    let t117758 = t116 * t31292;
    let t117765 = 3.0_f64 * t117 * t117575 * t572 + 6.0_f64 * t117103 * t1518 * t572 + 12.0_f64 * t117758 * t572 * t670 + 6.0_f64 * t13514 * t572 * t8295 + 6.0_f64 * t2327 * t572 * t8362 + 6.0_f64 * t2371 * t31370 * t572 + 12.0_f64 * t31117 * t4292 * t572 + 12.0_f64 * t1459 * t31359 + 12.0_f64 * t1459 * t31362 + 12.0_f64 * t1459 * t31365 + 12.0_f64 * t1459 * t31371 + 6.0_f64 * t1459 * t31374 + 6.0_f64 * t1461 * t31340 + 6.0_f64 * t1916 * t31114 + 3.0_f64 * t1916 * t31124 + 6.0_f64 * t4158 * t8383 + 3.0_f64 * t4158 * t8386 + 12.0_f64 * t5802 * t8289;
    t117765
}
