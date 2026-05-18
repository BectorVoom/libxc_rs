//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1246/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1246<F: Float>(t1009: F, t9372: F, t14443: F, t26680: F, t7703: F, t26692: F, t26757: F, t2836: F, t982: F, t990: F, t26854: F, t7696: F) -> (F, F, F, F, F) {
    let t93485 = t1009 * t9372;
    let t93526 = t7703 * t14443 * t26680;
    let t93542 = t26692 * t26757;
    let t93562 = t2836 * t982 * t990;
    let t93569 = t7696 * t26854;
    (t93485, t93526, t93542, t93562, t93569)
}
