//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2077/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2077<F: Float>(t6746: F, t82655: F, t884: F, t23384: F, t23715: F, t210: F, t23632: F, t23668: F, t23628: F, t6680: F, t23669: F, t995: F) -> (F, F, F, F, F) {
    let t82657 = t82655 * t884 * t6746;
    let t82661 = t23384 * t23715;
    let t82668 = t23668 * t210 * t23632;
    let t82694 = t6680 * t23628;
    let t82713 = t23669 * t995;
    (t82657, t82661, t82668, t82694, t82713)
}
