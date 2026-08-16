//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1022/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1022(t11509: f64, t23648: f64, t981: f64, t23448: f64, t23450: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23554: f64, t23556: f64) -> (f64, f64) {
    let t23649 = t23648 * t11509;
    let t23651 = 0.10254018858216406658e4_f64 * t981 * t23649;
    let t23652 = t23461 + t23463 + t23465 - t23469 + t23549 + t23552 - t23651 + t23448 - t23554 - t23556 - t23450;
    (t23651, t23652)
}
