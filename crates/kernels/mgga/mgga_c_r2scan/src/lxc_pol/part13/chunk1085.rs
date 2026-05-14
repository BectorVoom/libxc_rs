//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1085/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1085<F: Float>(t14160: F, t40630: F, t40631: F, t11550: F, t792: F, t3262: F, t3276: F, t10648: F, t10971: F, t11564: F, t481: F, t10610: F, t3263: F, t14656: F, t795: F, t3270: F) -> (F, F, F, F, F) {
    let t40634 = 3.0 * t40630 * t14160 * t40631;
    let t40635 = t11550 * t792;
    let t40638 = 15.0 / 8.0 * t3262 * t3276 * t40635;
    let t40642 = t10648 * t10971 * t11564;
    let t40644 = t11550 * t481;
    let t40647 = 3.0 * t10610 * t3263 * t40644;
    let t40648 = t14656 * t795;
    let t40649 = t3270 * t40648;
    (t40634, t40638, t40642, t40647, t40649)
}
