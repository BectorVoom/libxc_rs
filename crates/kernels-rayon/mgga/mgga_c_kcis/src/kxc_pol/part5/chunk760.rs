//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 760/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk760(t1437: f64, t5477: f64, t1451: f64, t1430: f64, t5441: f64, t5427: f64, t542: f64, t1330: f64, t104: f64, t111: f64, t120: f64, t4054: f64, t4055: f64, t4060: f64, t4062: f64, t4073: f64, t4858: f64, t4865: f64, t4881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5836 = t1437 * t5477;
    let t5839 = t1451 * t5477;
    let t5842 = t1430 * t5477;
    let t5845 = t1451 * t5441;
    let t5848 = t1430 * t5427;
    let t5851 = t1430 * t5441;
    let t5854 = t542 * t5427;
    let t5857 = t1437 * t5441;
    let t5860 = t1330 * t5427;
    let t5866 = -0.1585e-2_f64 * t4865 * t5836 - 0.10082625e-4_f64 * t4881 * t5839 + 0.7026e-2_f64 * t4858 * t5842 - 0.10082625e-4_f64 * t120 * t5845 - 0.672175e-5_f64 * t120 * t5848 + 0.7026e-2_f64 * t104 * t5851 + 0.1171e-2_f64 * t104 * t5854 - 0.1585e-2_f64 * t111 * t5857 - 0.52833333333333333333e-3_f64 * t111 * t5860 - 0.117630625e-4_f64 * t4073 - 0.11955719325063177623e-1_f64 * t4055 + 0.10359077815592613752e-3_f64 * t4062 - t4054 + t4060;
    (t5836, t5839, t5842, t5845, t5848, t5851, t5854, t5857, t5860, t5866)
}
