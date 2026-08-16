//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1157/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1157(t18648: f64, t2970: f64, t18657: f64, t945: f64, t1064: f64, t18570: f64, t18653: f64, t18574: f64, t1079: f64, t1056: f64, t1030: f64, t104: f64, t1072: f64, t111: f64, t120: f64, t18443: f64, t19473: f64, t19476: f64, t19478: f64, t19480: f64, t19482: f64, t19488: f64, t19491: f64, t4858: f64, t4865: f64, t4881: f64) -> f64 {
    let t19494 = t2970 * t18648;
    let t19497 = t945 * t18657;
    let t19500 = t1064 * t18570;
    let t19503 = t945 * t18653;
    let t19506 = t1064 * t18574;
    let t19509 = t1079 * t18570;
    let t19512 = t1056 * t18653;
    let t19515 = t1079 * t18574;
    let t19518 = t1056 * t18570;
    let t19521 = 0.28104e-1_f64 * t4858 * t19473 - 0.31077233446777841256e-3_f64 * t19476 - 0.11955719325063177623e-1_f64 * t19478 + 0.10359077815592613752e-3_f64 * t19480 + 0.23911438650126355246e-1_f64 * t19482 + 0.11955719325063177623e-1_f64 * t1030 * t18443 - 0.5179538907796306876e-4_f64 * t1072 * t18443 - 0.1585e-2_f64 * t111 * t19488 - 0.52833333333333333333e-3_f64 * t111 * t19491 - 0.17611111111111111111e-3_f64 * t111 * t19494 - 0.21133333333333333333e-2_f64 * t4865 * t19497 + 0.4755e-2_f64 * t111 * t19500 + 0.317e-2_f64 * t111 * t19503 - 0.634e-2_f64 * t4865 * t19506 + 0.30247875e-4_f64 * t120 * t19509 + 0.403305e-4_f64 * t120 * t19512 - 0.403305e-4_f64 * t4881 * t19515 - 0.21078e-1_f64 * t104 * t19518;
    t19521
}
