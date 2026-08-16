//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1047/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1047<F: Float>(t3416: F, t6755: F, t1096: F, t19309: F, t113: F, t3268: F, t97: F, t10666: F, t3347: F, t5086: F, t1064: F, t23040: F) -> (F, F, F, F, F, F) {
    let t37223 = t6755 * t3416;
    let t37226 = t19309 * t1096;
    let t37271 = t97 * t3268 * t113;
    let t37282 = t97 * t10666 * t113;
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    (t37223, t37226, t37271, t37282, t37292, t37299)
}
