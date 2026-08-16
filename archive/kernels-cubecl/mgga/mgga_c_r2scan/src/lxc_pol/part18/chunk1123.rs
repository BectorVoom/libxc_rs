//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1123/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1123<F: Float>(t14656: F, t795: F, t3270: F, t10966: F, t1103: F, t269: F, t955: F, t11505: F, t494: F, t97: F, t3446: F, t37475: F, t970: F) -> (F, F, F, F) {
    let t40648 = t14656 * t795;
    let t40649 = t3270 * t40648;
    let t40659 = t10966 * t1103 * t955 * t269;
    let t40664 = t97 * t11505 * t494;
    let t40672 = t3446 * t37475 * t970;
    (t40649, t40659, t40664, t40672)
}
