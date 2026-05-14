//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 790/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk790<F: Float>(t1173: F, t1337: F, t459: F, t1163: F, t3559: F, t1175: F, t3579: F, t3539: F, t1354: F, t1422: F, t3593: F, t1364: F, t3544: F, t306: F, t3529: F, t3575: F) -> (F, F, F, F, F, F) {
    let t13129 = t1337 * t1173 * t459;
    let t13130 = t1163 * t3559;
    let t13131 = t13129 * t13130;
    let t13134 = t3579 * t1175;
    let t13135 = t3539 * t13134;
    let t13138 = t1422 * t1354;
    let t13139 = t1163 * t3593;
    let t13140 = t13138 * t13139;
    let t13143 = t3579 * t1364;
    let t13144 = t3544 * t13143;
    let t13148 = t3529 * t306 * t459;
    let t13149 = t3575 * t1175;
    (t13131, t13135, t13140, t13144, t13148, t13149)
}
