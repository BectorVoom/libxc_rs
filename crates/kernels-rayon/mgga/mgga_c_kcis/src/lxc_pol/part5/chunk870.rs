//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 870/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk870(t1451: f64, t6912: f64, t1430: f64, t6944: f64, t542: f64, t6937: f64, t1437: f64, t1330: f64, t104: f64, t111: f64, t120: f64, t1404: f64, t1445: f64, t4093: f64, t6281: f64, t6284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7158 = t1451 * t6912;
    let t7161 = t1430 * t6944;
    let t7164 = t542 * t6937;
    let t7167 = t1437 * t6944;
    let t7170 = t1330 * t6937;
    let t7173 = t1451 * t6944;
    let t7176 = t1430 * t6937;
    let t7183 = t1430 * t6912;
    let t7186 = t1437 * t6912;
    let t7189 = 0.15538616723388920628e-3_f64 * t4093 * t6281 - 0.10082625e-4_f64 * t120 * t7158 - 0.3513e-2_f64 * t104 * t7161 + 0.1171e-2_f64 * t104 * t7164 + 0.7925e-3_f64 * t111 * t7167 - 0.52833333333333333333e-3_f64 * t111 * t7170 + 0.50413125e-5_f64 * t120 * t7173 - 0.672175e-5_f64 * t120 * t7176 + 0.11955719325063177623e-1_f64 * t1404 * t6284 - 0.5179538907796306876e-4_f64 * t1445 * t6284 + 0.7026e-2_f64 * t104 * t7183 - 0.1585e-2_f64 * t111 * t7186;
    (t7158, t7161, t7164, t7167, t7170, t7173, t7176, t7183, t7186, t7189)
}
