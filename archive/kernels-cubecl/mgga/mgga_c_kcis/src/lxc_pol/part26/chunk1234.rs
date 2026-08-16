//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1234/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1234<F: Float>(t4479: F, t7996: F, t12344: F, t2247: F, t1598: F, t251: F, t40512: F, t40515: F, t617: F, t40484: F, t27566: F, t27606: F) -> (F, F, F, F, F, F) {
    let t94824 = t7996 * t4479;
    let t94833 = t2247 * t12344;
    let t94861 = t40512 * t251 * t1598;
    let t94862 = t617 * t40515;
    let t94901 = t40484 * t251 * t1598;
    let t94928 = t27606 * t27566;
    (t94824, t94833, t94861, t94862, t94901, t94928)
}
