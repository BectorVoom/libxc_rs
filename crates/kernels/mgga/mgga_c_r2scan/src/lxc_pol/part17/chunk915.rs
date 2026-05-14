//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 915/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk915<F: Float>(t12495: F, t6093: F, t261: F, t3217: F, t3299: F, t11748: F, t3594: F, t3223: F, t3304: F, t2124: F, t9318: F, t3295: F, t3308: F, t9296: F, t1577: F, t2651: F, t3597: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12496 = t6093 * t12495;
    let t12498 = t261 * t3217;
    let t12499 = t3299 * t12498;
    let t12501 = t11748 * t3594;
    let t12503 = t261 * t3223;
    let t12504 = t3304 * t12503;
    let t12506 = t2124 * t9318;
    let t12507 = t3295 * t12506;
    let t12511 = t3308 * t9296;
    let t12512 = t1577 * t12511;
    let t12515 = t2651 * t3597;
    (t12496, t12498, t12499, t12501, t12503, t12504, t12506, t12507, t12511, t12512, t12515)
}
