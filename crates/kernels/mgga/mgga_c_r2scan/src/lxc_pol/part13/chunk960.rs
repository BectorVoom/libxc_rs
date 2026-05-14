//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 960/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk960<F: Float>(t37541: F, t10913: F, t1561: F, t10954: F, t10958: F, t3446: F, t10962: F, t10949: F, t2312: F, t3447: F, t3438: F, t6868: F, t10966: F, t1103: F, t269: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t37542 = 0.24390119833260022651e-2 * t37541;
    let t37543 = t1561 * t10913;
    let t37556 = t3446 * t10954 * t10958;
    let t37560 = t3446 * t10954 * t10962;
    let t37561 = 0.12195059916630011326e-2 * t37560;
    let t37564 = t3446 * t3447 * t10949 * t2312;
    let t37568 = t3446 * t3447 * t3438 * t6868;
    let t37569 = 0.15243824895787514157e-3 * t37568;
    let t37580 = t10966 * t1103 * t607 * t269;
    (t37542, t37543, t37556, t37561, t37564, t37569, t37580)
}
