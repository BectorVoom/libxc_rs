//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3012/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3012(t1063: f64, t1066: f64, t11698: f64, t11707: f64, t11977: f64, t15618: f64, t15850: f64, t16070: f64, t247: f64, t3177: f64, t43172: f64, t4869: f64, t51969: f64, t55046: f64, t55058: f64, t55062: f64, t55065: f64, t55067: f64) -> f64 {
    let t55069 = -0.28582678745379824648e-3_f64 * t43172 + 0.42874018118069736972e-3_f64 * t15618 * t11698 + 0.7145669686344956162e-3_f64 * t15618 * t11707 - 0.34299214494455789577e-2_f64 * t55046 * t16070 + 0.14291339372689912324e-3_f64 * t1063 * t247 * t1066 * t51969 + 0.42874018118069736972e-3_f64 * t15850 * t3177 - 0.68598428988911579154e-2_f64 * t11977 * t4869 - 0.28582678745379824648e-3_f64 * t55058 + t55062 - t55065 - 0.85748036236139473944e-3_f64 * t55067;
    t55069
}
