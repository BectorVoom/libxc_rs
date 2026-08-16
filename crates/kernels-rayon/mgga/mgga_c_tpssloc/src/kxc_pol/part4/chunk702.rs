//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 702/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk702(t1147: f64, t1687: f64, t1155: f64, t1695: f64, t3238: f64, t3295: f64, t3383: f64, t3390: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64) -> (f64, f64, f64) {
    let t4835 = t1687 * t1147;
    let t4840 = t1695 * t1155;
    let t4857 = -0.1294625e1_f64 * t4749 + 0.258925e1_f64 * t4757 + t3383 - 0.10064166666666666667e0_f64 * t3238 - 0.10064166666666666667e0_f64 * t4721 - 0.20128333333333333333e0_f64 * t4726 + 0.60385e0_f64 * t4731 + 0.301925e0_f64 * t4735 + 0.82524375e-1_f64 * t4765 + 0.16504875e0_f64 * t4767 + t3390 - 0.5519e-1_f64 * t3295 - 0.5519e-1_f64 * t4770 - 0.27595e-1_f64 * t4773 + 0.16557e0_f64 * t4776 + 0.82785e-1_f64 * t4779;
    (t4835, t4840, t4857)
}
