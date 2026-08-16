//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2096/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2096<F: Float>(t23384: F, t23582: F, t23333: F, t82431: F, t23323: F, t6683: F, t23357: F, t6680: F, t23494: F, t381: F, t23403: F, t23589: F) -> (F, F, F, F, F, F, F) {
    let t83318 = t23384 * t23582;
    let t83329 = t82431 * t23333;
    let t83342 = t23323 * t6683;
    let t83344 = t6680 * t23357;
    let t83352 = t23494 * t381;
    let t83358 = t23384 * t23403;
    let t83364 = t23384 * t23589;
    (t83318, t83329, t83342, t83344, t83352, t83358, t83364)
}
