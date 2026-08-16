//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1573/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1573(t19872: f64, t3092: f64, t1062: f64, t15670: f64, t247: f64, t3109: f64, t6096: f64, t1063: f64, t11672: f64, t11774: f64, t15796: f64, t15829: f64, t19858: f64, t19861: f64, t19864: f64, t19867: f64, t19869: f64, t3091: f64, t375: f64, t4839: f64, t6268: f64) -> f64 {
    let t19873 = t3092 * t19872;
    let t19878 = t15670 * t1062;
    let t19882 = t247 * t3109 * t6096;
    let t19883 = t1063 * t19882;
    let t19885 = -t15796 + 0.21437009059034868486e-3_f64 * t19858 * t375 + t15829 - 0.28582678745379824648e-3_f64 * t11774 * t19861 - 0.28582678745379824648e-3_f64 * t11774 * t19864 + 0.14291339372689912324e-3_f64 * t19867 - 0.11433071498151929859e-2_f64 * t19869 * t375 - 0.28582678745379824648e-3_f64 * t3091 * t19873 - 0.15244095330869239812e-2_f64 * t11672 * t6268 + 0.85748036236139473944e-3_f64 * t19878 * t4839 - 0.19055119163586549765e-3_f64 * t19883;
    t19885
}
