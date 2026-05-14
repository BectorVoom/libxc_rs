//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1143/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1143<F: Float>(t247: F, t3109: F, t6096: F, t1063: F, t11672: F, t11774: F, t15796: F, t15829: F, t19858: F, t19861: F, t19864: F, t19867: F, t19869: F, t19873: F, t19878: F, t3091: F, t375: F, t4839: F, t6268: F) -> (F,) {
    let t19882 = t247 * t3109 * t6096;
    let t19883 = t1063 * t19882;
    let t19885 = -t15796 + 0.21437009059034868486e-3 * t19858 * t375 + t15829 - 0.28582678745379824648e-3 * t11774 * t19861 - 0.28582678745379824648e-3 * t11774 * t19864 + 0.14291339372689912324e-3 * t19867 - 0.11433071498151929859e-2 * t19869 * t375 - 0.28582678745379824648e-3 * t3091 * t19873 - 0.15244095330869239812e-2 * t11672 * t6268 + 0.85748036236139473944e-3 * t19878 * t4839 - 0.19055119163586549765e-3 * t19883;
    (t19885,)
}
