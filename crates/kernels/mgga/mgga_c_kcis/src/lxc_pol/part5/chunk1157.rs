//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1157/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1157<F: Float>(t18648: F, t2970: F, t18657: F, t945: F, t1064: F, t18570: F, t18653: F, t18574: F, t1079: F, t1056: F, t1030: F, t104: F, t1072: F, t111: F, t120: F, t18443: F, t19473: F, t19476: F, t19478: F, t19480: F, t19482: F, t19488: F, t19491: F, t4858: F, t4865: F, t4881: F) -> F {
    let t19494 = t2970 * t18648;
    let t19497 = t945 * t18657;
    let t19500 = t1064 * t18570;
    let t19503 = t945 * t18653;
    let t19506 = t1064 * t18574;
    let t19509 = t1079 * t18570;
    let t19512 = t1056 * t18653;
    let t19515 = t1079 * t18574;
    let t19518 = t1056 * t18570;
    let t19521 = F::new(0.28104e-1) * t4858 * t19473 - F::cast_from(0.31077233446777841256e-3_f64) * t19476 - F::cast_from(0.11955719325063177623e-1_f64) * t19478 + F::cast_from(0.10359077815592613752e-3_f64) * t19480 + F::cast_from(0.23911438650126355246e-1_f64) * t19482 + F::cast_from(0.11955719325063177623e-1_f64) * t1030 * t18443 - F::cast_from(0.5179538907796306876e-4_f64) * t1072 * t18443 - F::new(0.1585e-2) * t111 * t19488 - F::cast_from(0.52833333333333333333e-3_f64) * t111 * t19491 - F::cast_from(0.17611111111111111111e-3_f64) * t111 * t19494 - F::cast_from(0.21133333333333333333e-2_f64) * t4865 * t19497 + F::new(0.4755e-2) * t111 * t19500 + F::new(0.317e-2) * t111 * t19503 - F::new(0.634e-2) * t4865 * t19506 + F::new(0.30247875e-4) * t120 * t19509 + F::new(0.403305e-4) * t120 * t19512 - F::new(0.403305e-4) * t4881 * t19515 - F::new(0.21078e-1) * t104 * t19518;
    t19521
}
