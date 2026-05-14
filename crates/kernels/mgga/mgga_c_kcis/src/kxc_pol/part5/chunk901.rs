//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 901/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk901<F: Float>(t1409: F, t1471: F, t1317: F, t1392: F, t544: F, t3751: F, t456: F, t3752: F, t518: F, t10269: F, t546: F, t3943: F, t478: F, t463: F, t1075: F, t237: F, t451: F) -> (F, F, F, F, F, F, F, F) {
    let t11322 = t1471 * t1409;
    let t11332 = t1392 * t1317 * t544;
    let t11369 = t3751 * t456 * t544;
    let t11374 = t3752 * t518;
    let t11384 = 0.29201909629629629629e-3 * t10269 * t546;
    let t11388 = 1.0 / t3943 / t478;
    let t11402 = 1.0 / t456 / t463 / 4.0;
    let t11407 = t237 * t1075 * t451;
    (t11322, t11332, t11369, t11374, t11384, t11388, t11402, t11407)
}
