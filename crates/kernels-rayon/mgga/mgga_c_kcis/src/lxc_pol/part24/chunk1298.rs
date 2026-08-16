//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1298/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1298(t101355: f64, t7690: f64, t1704: f64, t1709: f64, t922: f64, t93426: f64, t100596: f64, t100599: f64, t100602: f64, t101195: f64, t28997: f64, t7687: f64, t8034: f64, t92993: f64, t92997: f64, t93425: f64, t93592: f64, t96026: f64, t96522: f64) -> (f64, f64) {
    let t101372 = t7690 * t101355;
    let t101374 = t1704 * t1709;
    let t101376 = t93426 * t101374 * t922;
    let t101383 = 0.16581944444444444444e-2_f64 * t100596 + 0.16581944444444444444e-2_f64 * t100599 + 0.33163888888888888888e-2_f64 * t100602 - 0.13901041666666666667e-2_f64 * t7687 * t28997 + 0.18550940104166666667e-3_f64 * t96522 * t8034 - 0.92754700520833333333e-4_f64 * t101372 - 0.46336805555555555557e-3_f64 * t93592 * t101376 + 0.82448622685185185185e-4_f64 * t93425 * t101195 + t96026 + 0.55273148148148148147e-3_f64 * t92993 - 0.36848765432098765431e-3_f64 * t92997;
    (t101376, t101383)
}
