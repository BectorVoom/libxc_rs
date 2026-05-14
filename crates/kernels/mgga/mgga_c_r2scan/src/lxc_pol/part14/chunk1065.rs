//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1065/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1065<F: Float>(t11336: F, t2850: F, t3270: F, t3269: F, t3262: F, t3465: F, t40579: F, t23754: F, t3275: F, t11325: F, t11555: F, t3582: F, t38718: F, t3579: F, t38749: F, t11559: F) -> (F, F, F, F, F, F, F) {
    let t41298 = t3270 * t11336 * t2850;
    let t41300 = t3269 * t41298 / 2.0;
    let t41305 = 3.0 / 4.0 * t3262 * t3465 * t40579;
    let t41308 = t3275 * t3465 * t23754 / 4.0;
    let t41311 = 5.0 / 8.0 * t3275 * t11325 * t11555;
    let t41314 = 5.0 / 16.0 * t3275 * t38718 * t3582;
    let t41316 = 5.0 / 16.0 * t3579 * t38749;
    let t41319 = 5.0 / 8.0 * t3275 * t11325 * t11559;
    (t41300, t41305, t41308, t41311, t41314, t41316, t41319)
}
