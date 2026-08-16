//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1109/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1109(t1207: f64, t456: f64, t487: f64, t1269: f64, t3566: f64, t1203: f64, t3565: f64, t3552: f64, t1208: f64, t3551: f64, t1209: f64, t3727: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    let t12658 = t12657 * t487;
    let t12666 = t1209 * t3727;
    (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666)
}
