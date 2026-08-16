//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1197/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1197(t23640: f64, t373: f64, t11257: f64, t1042: f64, t11506: f64, t23451: f64, t11509: f64, t981: f64, t23448: f64, t23450: f64, t23461: f64, t23463: f64, t23465: f64, t23469: f64, t23549: f64, t23552: f64, t23554: f64, t23556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23641 = t373 * t23640;
    let t23642 = t23641 * t11257;
    let t23643 = t1042 * t23642;
    let t23648 = t11506 * t23451;
    let t23649 = t23648 * t11509;
    let t23651 = 0.10254018858216406658e4_f64 * t981 * t23649;
    let t23652 = t23461 + t23463 + t23465 - t23469 + t23549 + t23552 - t23651 + t23448 - t23554 - t23556 - t23450;
    (t23641, t23642, t23643, t23649, t23651, t23652)
}
