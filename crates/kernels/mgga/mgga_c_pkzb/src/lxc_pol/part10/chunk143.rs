//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 143/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk143<F: Float>(t411: F, t415: F, t135: F, t273: F, t354: F, t382: F, t384: F) -> (F, F) {
    let t418 = 1.0 + 0.65854491829355115987e0 * t411 * t415;
    let t419 = f64::ln(t418);
    let t422 = t135 * t273 * t419 - t354 + t382 + t384;
    (t418, t422)
}
