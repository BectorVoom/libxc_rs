//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 751/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk751<F: Float>(t2127: F, t6118: F, t1550: F, t1569: F, t2597: F, t546: F, t1553: F, t277: F, t565: F, t1582: F, t259: F, t503: F, t6068: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6119 = t6118 * t2127;
    let t6127 = t1569 * t1550;
    let t6132 = t546 * t2597;
    let t6133 = t277 * t1553;
    let t6139 = t565 * t2597;
    let t6148 = t1582 * t259;
    let t6149 = t546 * t6148;
    let t6152 = t565 * t6148;
    let t6155 = t503 * t6068;
    (t6119, t6127, t6132, t6133, t6139, t6148, t6149, t6152, t6155)
}
