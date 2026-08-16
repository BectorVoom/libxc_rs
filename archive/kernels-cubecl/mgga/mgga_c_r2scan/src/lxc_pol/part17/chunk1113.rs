//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1113/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1113<F: Float>(t38145: F, t6085: F, t8081: F, t6093: F, t7619: F, t2147: F, t7624: F, t1575: F, t269: F, t546: F, t565: F, t10728: F, t7258: F) -> (F, F, F, F, F, F) {
    let t40047 = t6085 * t38145 * t8081;
    let t40050 = t6093 * t38145 * t7619;
    let t40053 = t2147 * t38145 * t7624;
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    let t40066 = t565 * t40061;
    let t40070 = t10728 * t7258;
    (t40047, t40050, t40053, t40062, t40066, t40070)
}
