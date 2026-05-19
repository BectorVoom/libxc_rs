//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 141/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk141<F: Float>(t24: F, t411: F, t415: F, t135: F, t273: F, t354: F, t382: F, t384: F, t30: F, t42: F, zeta_threshold: F) -> (F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t418 = F::new(1.0) + F::cast_from(0.65854491829355115987e0_f64) * t411 * t415;
    let t419 = F::ln(t418);
    let t422 = t135 * t273 * t419 - t354 + t382 + t384;
    let t423 = piecewise3::<F>(t90, zeta_threshold, t24);
    let t430 = t30 * t42;
    (t418, t422, t423, t430)
}
