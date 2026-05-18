//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 826/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk826<F: Float>(t3293: F, t6593: F, t1109: F, t6338: F, t345: F, t3303: F, t6316: F, t1114: F, t6352: F, t1697: F, t1102: F, t278: F, t3253: F, t344: F, t4563: F, t4592: F, t4630: F, t6432: F, t6570: F, t6574: F, t6578: F, t6582: F, t6586: F, t6590: F) -> (F, F, F, F, F, F, F, F) {
    let t6594 = t3293 * t6593;
    let t6597 = t1109 * t6338;
    let t6598 = t345 * t6597;
    let t6601 = t3303 * t6316;
    let t6602 = t345 * t6601;
    let t6605 = t1114 * t6352;
    let t6606 = t345 * t6605;
    let t6609 = t1697 * t1697;
    let t6613 = -t3253 + F::new(0.8760572888888888889e-3) * t4563 + F::new(0.19711289e-2) * t4592 - F::new(0.13140859333333333333e-2) * t4630 + F::new(0.10950716111111111111e-2) * t1102 * t6570 + F::new(0.19711289e-2) * t1102 * t6574 - F::new(0.13140859333333333333e-2) * t1102 * t6578 - F::new(0.13140859333333333333e-2) * t1102 * t6582 + F::new(0.65704296666666666667e-3) * t1102 * t6586 + F::new(0.7391733375e-3) * t344 * t6590 - F::new(0.295669335e-2) * t1102 * t6594 + F::new(0.1478346675e-2) * t344 * t6598 + F::new(0.19711289e-2) * t344 * t6602 - F::new(0.98556445e-3) * t344 * t6606 - F::new(4.0) * t6609 - F::new(4.0) * t278 * t6432;
    (t6594, t6597, t6598, t6601, t6602, t6605, t6606, t6613)
}
