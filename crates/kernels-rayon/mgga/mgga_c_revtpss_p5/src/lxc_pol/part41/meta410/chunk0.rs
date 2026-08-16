//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1438/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1438(t9422: f64, t9559: f64, t9566: f64, t9570: f64, t9578: f64, t13643: f64, t9421: f64, t9427: f64, t9429: f64, t9514: f64, t9517: f64, t9521: f64, t9546: f64, t9569: f64, t9574: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22205 = 0.11696447245269292414e1_f64 * t9422;
    let t22206 = 20.0_f64 * t9559;
    let t22207 = 0.24415263074675393405e-3_f64 * t9566;
    let t22208 = 32.0_f64 * t9570;
    let t22209 = 12.0_f64 * t9578;
    let t22210 = t9421 + t22205 - t9427 + t9429 + t9546 + t22206 + t9514 - t13643 + t22207 - t9517 - t9521 + t9569 + t22208 - t9574 - t9577 + t22209;
    (t22205, t22206, t22207, t22208, t22209, t22210)
}
