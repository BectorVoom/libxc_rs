//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1032/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1032<F: Float>(t19243: F, t9924: F, t25: F, t6540: F, t285: F, t14538: F, t19219: F, t19223: F, t19226: F, t19229: F, t19233: F, t19236: F, t19239: F, t2872: F, t6522: F, t6541: F, t984: F, t991: F) -> (F,) {
    let t19244 = t9924 * t19243;
    let t19249 = t25 * t6540;
    let t19250 = t285 * t19249;
    let t19252 = t2872 * t6522 / 54.0 - t991 * t19219 / 72.0 + t991 * t19223 / 144.0 + t991 * t19226 / 48.0 - t991 * t19229 / 36.0 - t991 * t19233 / 288.0 - t991 * t19236 / 144.0 + t991 * t19239 / 216.0 + t991 * t19244 / 144.0 - t14538 + t984 * t6541 / 36.0 - t19250 / 288.0;
    (t19252,)
}
