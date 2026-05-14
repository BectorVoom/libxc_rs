//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 963/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk963<F: Float>(t10856: F, t5116: F, t10707: F, t1591: F, t10710: F, t20238: F, t10810: F, t1592: F, t6166: F, t10811: F, t1584: F, t5169: F, t1583: F, t546: F, t2078: F, t3320: F, t783: F, t787: F) -> (F, F, F, F, F, F, F, F) {
    let t37656 = t10856 * t5116;
    let t37658 = t1591 * t10707;
    let t37660 = t37658 * t10710 * t20238;
    let t37674 = t1592 * t10810 * t6166;
    let t37676 = t1584 * t10811;
    let t37681 = t10856 * t5169;
    let t37685 = t546 * t1583;
    let t37696 = t783 * t2078 * t787 * t3320;
    (t37656, t37658, t37660, t37674, t37676, t37681, t37685, t37696)
}
