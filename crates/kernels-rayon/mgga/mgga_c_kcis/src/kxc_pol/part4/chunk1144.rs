//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1144/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1144(t14522: f64, t417: f64, t2872: f64, t4936: f64, t1699: f64, t9916: f64, t991: f64, t14486: f64, t14489: f64, t14493: f64, t14498: f64, t14502: f64, t14513: f64, t14518: f64, t1700: f64, t4940: f64, t4944: f64, t4948: f64, t9903: f64) -> f64 {
    let t14523 = t417 * t14522;
    let t14527 = t2872 * t4936 / 162.0_f64;
    let t14528 = t9916 * t1699;
    let t14529 = t991 * t14528;
    let t14531 = -t991 * t14486 / 144.0_f64 + t991 * t14489 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t991 * t14493 + t991 * t14498 / 54.0_f64 - t991 * t14502 / 288.0_f64 + t2872 * t4944 / 54.0_f64 + t2872 * t4948 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t2872 * t4940 + t991 * t14513 / 24.0_f64 + t14518 + 11.0_f64 / 324.0_f64 * t9903 * t1700 - t991 * t14523 / 16.0_f64 - t14527 - t14529 / 1296.0_f64;
    t14531
}
