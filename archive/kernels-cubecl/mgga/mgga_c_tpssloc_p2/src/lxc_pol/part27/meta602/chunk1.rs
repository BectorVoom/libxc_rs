//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2072/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2072<F: Float>(t131: F, t350: F, t38: F, t82510: F, t10469: F, t344: F, t10482: F, t3032: F, t2261: F, t6794: F, t23598: F, t614: F) -> (F, F, F, F, F) {
    let t82513 = t38 * t82510 * t131 * t350;
    let t82514 = t344 * t10469;
    let t82516 = t3032 * t10482;
    let t82527 = t2261 * t6794 * t131 * t350;
    let t82534 = t614 * t23598 * t131 * t350;
    (t82513, t82514, t82516, t82527, t82534)
}
