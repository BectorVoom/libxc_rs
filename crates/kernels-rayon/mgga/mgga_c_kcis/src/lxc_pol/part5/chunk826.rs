//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 826/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk826(t3293: f64, t6593: f64, t1109: f64, t6338: f64, t345: f64, t3303: f64, t6316: f64, t1114: f64, t6352: f64, t1697: f64, t1102: f64, t278: f64, t3253: f64, t344: f64, t4563: f64, t4592: f64, t4630: f64, t6432: f64, t6570: f64, t6574: f64, t6578: f64, t6582: f64, t6586: f64, t6590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6594 = t3293 * t6593;
    let t6597 = t1109 * t6338;
    let t6598 = t345 * t6597;
    let t6601 = t3303 * t6316;
    let t6602 = t345 * t6601;
    let t6605 = t1114 * t6352;
    let t6606 = t345 * t6605;
    let t6609 = t1697 * t1697;
    let t6613 = -t3253 + 0.8760572888888888889e-3_f64 * t4563 + 0.19711289e-2_f64 * t4592 - 0.13140859333333333333e-2_f64 * t4630 + 0.10950716111111111111e-2_f64 * t1102 * t6570 + 0.19711289e-2_f64 * t1102 * t6574 - 0.13140859333333333333e-2_f64 * t1102 * t6578 - 0.13140859333333333333e-2_f64 * t1102 * t6582 + 0.65704296666666666667e-3_f64 * t1102 * t6586 + 0.7391733375e-3_f64 * t344 * t6590 - 0.295669335e-2_f64 * t1102 * t6594 + 0.1478346675e-2_f64 * t344 * t6598 + 0.19711289e-2_f64 * t344 * t6602 - 0.98556445e-3_f64 * t344 * t6606 - 4.0_f64 * t6609 - 4.0_f64 * t278 * t6432;
    (t6594, t6597, t6598, t6601, t6602, t6605, t6606, t6613)
}
