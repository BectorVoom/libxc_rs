//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2968/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2968<F: Float>(t16095: F, t16127: F, t43131: F, t16088: F, t3046: F, t380: F, t16139: F, t3127: F, t3172: F, t1042: F, t11933: F, t15922: F, t16089: F, t16098: F, t16152: F, t2853: F, t3092: F, t3181: F, t42637: F, t42656: F, t42658: F, t42660: F, t42662: F, t4772: F, t906: F) -> (F, F) {
    let t54085 = t16095 * t43131 * t16127;
    let t54089 = t3046 * t380 * t16088;
    let t54099 = t3127 * t3172 * t16139;
    let t54110 = -F::cast_from(0.95275595817932748827e-3_f64) * t54085 + F::cast_from(0.57165357490759649295e-3_f64) * t42637 + F::cast_from(0.17149607247227894789e-2_f64) * t54089 * t16098 + F::cast_from(0.17149607247227894789e-2_f64) * t16089 * t3092 * t16152 * t906 + F::cast_from(0.68598428988911579154e-2_f64) * t11933 * t15922 - F::cast_from(0.57165357490759649295e-3_f64) * t54099 + F::cast_from(0.15244095330869239812e-2_f64) * t42656 - F::cast_from(0.45732285992607719436e-2_f64) * t42658 - F::cast_from(0.45732285992607719436e-2_f64) * t42660 + F::cast_from(0.22866142996303859718e-2_f64) * t42662 - F::cast_from(0.7145669686344956162e-3_f64) * t3127 * t1042 * t3181 * t4772 * t2853;
    (t54089, t54110)
}
