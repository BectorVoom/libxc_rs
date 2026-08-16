//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3012/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3012<F: Float>(t1063: F, t1066: F, t11698: F, t11707: F, t11977: F, t15618: F, t15850: F, t16070: F, t247: F, t3177: F, t43172: F, t4869: F, t51969: F, t55046: F, t55058: F, t55062: F, t55065: F, t55067: F) -> F {
    let t55069 = -F::cast_from(0.28582678745379824648e-3_f64) * t43172 + F::cast_from(0.42874018118069736972e-3_f64) * t15618 * t11698 + F::cast_from(0.7145669686344956162e-3_f64) * t15618 * t11707 - F::cast_from(0.34299214494455789577e-2_f64) * t55046 * t16070 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t247 * t1066 * t51969 + F::cast_from(0.42874018118069736972e-3_f64) * t15850 * t3177 - F::cast_from(0.68598428988911579154e-2_f64) * t11977 * t4869 - F::cast_from(0.28582678745379824648e-3_f64) * t55058 + t55062 - t55065 - F::cast_from(0.85748036236139473944e-3_f64) * t55067;
    t55069
}
