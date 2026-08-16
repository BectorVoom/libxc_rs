//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1787/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1787<F: Float>(t22886: F, t22892: F, t22893: F, t22751: F, t22887: F, t268: F, t547: F, t6559: F) -> (F, F, F) {
    let t81216 = t22892 * t22893 * t22886;
    let t81218 = t22751 * t22887;
    let t81228 = t6559 * t547 * t268;
    (t81216, t81218, t81228)
}
