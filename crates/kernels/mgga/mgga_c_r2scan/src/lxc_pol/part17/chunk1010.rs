//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1010/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1010<F: Float>(t2124: F, t9318: F, t3295: F, t3308: F, t9296: F, t1577: F, t2651: F, t3597: F, t9292: F, t574: F, t9445: F, t9422: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12506 = t2124 * t9318;
    let t12507 = t3295 * t12506;
    let t12511 = t3308 * t9296;
    let t12512 = t1577 * t12511;
    let t12515 = t2651 * t3597;
    let t12517 = t3308 * t9292;
    let t12518 = t574 * t12517;
    let t12520 = t3308 * t9445;
    let t12521 = t574 * t12520;
    let t12523 = t2124 * t9422;
    (t12506, t12507, t12511, t12512, t12515, t12517, t12518, t12520, t12521, t12523)
}
