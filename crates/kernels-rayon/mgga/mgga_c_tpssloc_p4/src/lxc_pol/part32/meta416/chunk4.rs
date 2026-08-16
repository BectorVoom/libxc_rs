//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1611/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1611(t1137: f64, t18893: f64, t1147: f64, t6063: f64, t1129: f64, t11303: f64, t11361: f64, t1138: f64, t11415: f64, t1157: f64, t15121: f64, t15141: f64, t1683: f64, t1695: f64, t18837: f64, t18839: f64, t18840: f64, t3327: f64, t4797: f64, t4820: f64, t4835: f64, t4858: f64, t6037: f64, t6053: f64, t6056: f64, t6088: f64) -> f64 {
    let t18894 = t18893 * t1137;
    let t18899 = t6063 * t1147;
    let t18906 = 0.17315859105681463759e2_f64 * t11361 * t6088 - t18837 - t18839 + 1.0_f64 * t18840 * t1138 + 2.0_f64 * t15141 * t1683 + 2.0_f64 * t4797 * t4820 - 2.0_f64 * t11303 * t6037 + 1.0_f64 * t3327 * t6053 + 1.0_f64 * t1129 * t18894 + 0.32163958997385070134e2_f64 * t11415 * t6056 + 0.5848223622634646207e0_f64 * t18899 * t1157 + 0.11696447245269292414e1_f64 * t15121 * t1695 + 0.11696447245269292414e1_f64 * t4835 * t4858;
    t18906
}
