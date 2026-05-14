//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 962/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk962<F: Float>(t2157: F, t625: F, t37637: F, t1583: F, t565: F, t2195: F, t573: F, t10856: F, t5116: F, t10707: F, t1591: F, t10710: F, t20238: F, t10810: F, t1592: F, t6166: F) -> (F, F, F, F, F, F, F, F) {
    let t37638 = t2157 * t625;
    let t37639 = t37637 * t37638;
    let t37641 = t565 * t1583;
    let t37652 = t2195 * t573;
    let t37656 = t10856 * t5116;
    let t37658 = t1591 * t10707;
    let t37660 = t37658 * t10710 * t20238;
    let t37674 = t1592 * t10810 * t6166;
    (t37638, t37639, t37641, t37652, t37656, t37658, t37660, t37674)
}
