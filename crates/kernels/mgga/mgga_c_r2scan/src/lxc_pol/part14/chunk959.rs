//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 959/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk959<F: Float>(t3434: F, t3439: F, t6860: F, t875: F, t10993: F, t3446: F, t502: F, t6876: F, t10954: F, t10958: F, t10962: F, t10949: F, t2312: F, t3447: F, t3438: F, t6868: F) -> (F, F, F, F, F, F) {
    let t37531 = t3434 * t6860 * t875 * t3439;
    let t37541 = t3446 * t502 * t6876 * t10993;
    let t37556 = t3446 * t10954 * t10958;
    let t37560 = t3446 * t10954 * t10962;
    let t37564 = t3446 * t3447 * t10949 * t2312;
    let t37568 = t3446 * t3447 * t3438 * t6868;
    (t37531, t37541, t37556, t37560, t37564, t37568)
}
