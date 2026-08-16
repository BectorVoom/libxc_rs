//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1295/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1295(t11509: f64, t2988: f64, t15541: f64, t981: f64, t15100: f64, t15103: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15399: f64, t15519: f64, t15522: f64, t15524: f64, t15528: f64, t15530: f64, t15536: f64, t15540: f64, t3329: f64, t5023: f64, t5024: f64) -> (f64, f64) {
    let t15542 = t11509 * t2988;
    let t15543 = t15541 * t15542;
    let t15545 = 0.10254018858216406658e4_f64 * t981 * t15543;
    let t15546 = -t3329 * t5023 * t5024 + t15100 - t15103 - t15377 + t15379 - t15382 - t15385 - t15388 + t15392 + t15395 + t15399 + t15519 + t15522 - t15524 - t15528 + t15530 - t15536 + t15540 - t15545;
    (t15545, t15546)
}
