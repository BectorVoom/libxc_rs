//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 977/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk977<F: Float>(t3751: F, t456: F, t544: F, t3752: F, t518: F, t10269: F, t546: F, t3943: F, t478: F, t463: F, t1075: F, t237: F, t451: F) -> (F, F, F, F, F, F) {
    let t11369 = t3751 * t456 * t544;
    let t11374 = t3752 * t518;
    let t11384 = F::new(0.29201909629629629629e-3) * t10269 * t546;
    let t11388 = F::new(1.0) / t3943 / t478;
    let t11402 = F::new(1.0) / t456 / t463 / F::new(4.0);
    let t11407 = t237 * t1075 * t451;
    (t11369, t11374, t11384, t11388, t11402, t11407)
}
