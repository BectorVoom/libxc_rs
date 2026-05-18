//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1169/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1169<F: Float>(t261: F, t3299: F, t9451: F, t31060: F, t3333: F, t12533: F, t22796: F, t1592: F, t29270: F, t3308: F, t2196: F, t29274: F) -> (F, F, F, F, F) {
    let t43238 = t3299 * t261 * t9451;
    let t43240 = t31060 * t3333;
    let t43242 = t22796 * t12533;
    let t43248 = t1592 * t3308 * t29270;
    let t43251 = t2196 * t3308 * t29274;
    (t43238, t43240, t43242, t43248, t43251)
}
