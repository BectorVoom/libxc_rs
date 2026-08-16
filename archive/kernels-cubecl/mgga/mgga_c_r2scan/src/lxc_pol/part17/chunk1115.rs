//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1115/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1115<F: Float>(t10833: F, t980: F, t38145: F, t6535: F, t8089: F, t261: F, t3299: F, t7386: F, t10869: F, t7601: F, t10811: F, t2651: F) -> (F, F, F, F, F) {
    let t40109 = t980 * t10833;
    let t40131 = t6535 * t38145 * t8089;
    let t40137 = t3299 * t261 * t7386;
    let t40155 = t7601 * t10869;
    let t40157 = t2651 * t10811;
    (t40109, t40131, t40137, t40155, t40157)
}
