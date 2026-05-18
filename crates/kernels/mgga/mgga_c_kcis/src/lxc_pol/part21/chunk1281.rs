//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1281/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1281<F: Float>(t27815: F, t7703: F, t9938: F, t14443: F, t27821: F, t1646: F, t2809: F, t4947: F, t93346: F, t1092: F, t14649: F, t3190: F, t7718: F) -> (F, F, F, F, F) {
    let t95605 = F::new(0.15445601851851851852e-3) * t7703 * t9938 * t27815;
    let t95606 = t14443 * t27821;
    let t95608 = F::new(0.15445601851851851852e-3) * t7703 * t95606;
    let t95621 = t4947 * t93346 * t1646 * t2809;
    let t95626 = t1092 * t7718 * t14649 * t3190;
    (t95605, t95606, t95608, t95621, t95626)
}
