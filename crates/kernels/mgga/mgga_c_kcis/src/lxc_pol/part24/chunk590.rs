//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 590/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk590<F: Float>(t345: F, t6605: F, t1697: F, t1102: F, t278: F, t3253: F, t344: F, t4563: F, t4592: F, t4630: F, t6432: F, t6570: F, t6574: F, t6578: F, t6582: F, t6586: F, t6590: F, t6594: F, t6598: F, t6602: F) -> (F, F) {
    let t6606 = t345 * t6605;
    let t6609 = t1697 * t1697;
    let t6613 = -t3253 + 0.8760572888888888889e-3 * t4563 + 0.19711289e-2 * t4592 - 0.13140859333333333333e-2 * t4630 + 0.10950716111111111111e-2 * t1102 * t6570 + 0.19711289e-2 * t1102 * t6574 - 0.13140859333333333333e-2 * t1102 * t6578 - 0.13140859333333333333e-2 * t1102 * t6582 + 0.65704296666666666667e-3 * t1102 * t6586 + 0.7391733375e-3 * t344 * t6590 - 0.295669335e-2 * t1102 * t6594 + 0.1478346675e-2 * t344 * t6598 + 0.19711289e-2 * t344 * t6602 - 0.98556445e-3 * t344 * t6606 - 4.0 * t6609 - 4.0 * t278 * t6432;
    (t6606, t6613)
}
