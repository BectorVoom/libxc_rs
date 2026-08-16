//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 327/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk327<F: Float>(t1066: F, t906: F, t247: F, t1003: F, t1009: F, t1011: F, t1017: F, t1021: F, t1025: F, t1028: F, t1041: F, t1047: F, t1054: F, t1060: F, t1063: F, t348: F, t375: F) -> (F, F) {
    let t1067 = t1066 * t906;
    let t1068 = t247 * t1067;
    let t1071 = -t1003 * t348 / F::cast_from(36.0_f64) + t1009 + t1011 * t1017 / F::cast_from(288.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t1021 * t375 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t1028 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1047 - F::cast_from(0.11433071498151929859e-2_f64) * t1054 * t375 + t1060 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t1068;
    (t1068, t1071)
}
