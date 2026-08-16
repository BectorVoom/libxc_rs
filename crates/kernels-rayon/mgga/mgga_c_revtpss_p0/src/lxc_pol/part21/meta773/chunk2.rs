//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2747/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747(t14769: f64, t2652: f64, t10716: f64, t14757: f64, t14772: f64, t221: f64, t2674: f64, t40683: f64, t10698: f64, t14494: f64, t14785: f64, t14917: f64, t2394: f64, t2745: f64, t40503: f64, t40507: f64, t40509: f64, t40511: f64, t40518: f64, t40523: f64, t40526: f64, t40529: f64, t40532: f64, t40535: f64, t40549: f64, t40553: f64, t40558: f64, t4343: f64, t828: f64, t851: f64) -> f64 {
    let t50529 = t2652 * t14769;
    let t50531 = t10716 * t14757;
    let t50532 = 0.8131200449485652516e-2_f64 * t50531;
    let t50538 = t221 * t14772;
    let t50540 = t2674 * t40683 * t50538;
    let t50558 = -0.60023625365297631762e-1_f64 * t50529 - t50532 - 0.77173232612525526549e-1_f64 * t851 * t10698 * t828 * t4343 * t2394 - 0.45738002528356795402e-2_f64 * t50540 + 0.85748036236139473944e-3_f64 * t40503 + t40507 + 0.76230004213927992336e-5_f64 * t40509 - 7.0_f64 / 16.0_f64 * t40511 - 0.13721400758507038621e-3_f64 * t40518 - 0.15246000842785598467e-4_f64 * t40523 - 0.54214778996945588148e-4_f64 * t40526 + 0.76230004213927992336e-5_f64 * t40529 + 0.27107389498472794074e-3_f64 * t40532 + 0.97586602194502058666e-3_f64 * t40535 + 0.42874018118069736972e-3_f64 * t40549 - 0.85748036236139473944e-4_f64 * t40553 - 0.85748036236139473944e-4_f64 * t40558 - 0.12862205435420921092e-1_f64 * t2745 * t14785 * t14494 * t14917;
    t50558
}
