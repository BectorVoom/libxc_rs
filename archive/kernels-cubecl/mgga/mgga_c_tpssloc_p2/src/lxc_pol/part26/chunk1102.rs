//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1102/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1102<F: Float>(t22882: F, t6637: F, t6888: F, t3719: F, t6968: F, t117: F, t547: F, t67: F, t6559: F) -> (F, F, F, F, F, F, F) {
    let t22883 = t6637 * t22882;
    let t22884 = t6888 * t22883;
    let t22886 = t6968 * t3719;
    let t22887 = t6637 * t22886;
    let t22888 = t6888 * t22887;
    let t22891 = t547 * t67 * t117;
    let t22892 = t6559 * t22891;
    (t22883, t22884, t22886, t22887, t22888, t22891, t22892)
}
