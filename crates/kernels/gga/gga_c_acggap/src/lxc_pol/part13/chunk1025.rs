//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1025/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1025<F: Float>(t1181: F, t2068: F, t20935: F, t604: F, t30269: F, t30297: F, t34072: F, t34074: F, t34076: F, t34077: F, t34078: F, t34082: F, t34085: F, t34089: F, t34092: F, t34095: F, t34100: F, t34102: F, t34105: F, t34107: F, t34111: F) -> F {
    let t34115 = t2068 * t1181 * t604 * t20935;
    let t34117 = F::cast_from(0.94344276868812456204e-2_f64) * t30269 - F::cast_from(0.68598428988911579156e-2_f64) * t34072 + F::cast_from(0.34299214494455789578e-2_f64) * t34074 + t34076 + t34077 - F::cast_from(0.34299214494455789578e-2_f64) * t34078 - F::cast_from(0.21437009059034868486e-2_f64) * t30297 - t34082 - F::cast_from(0.15724046144802076034e-2_f64) * t34085 - F::cast_from(0.10718504529517434243e-2_f64) * t34089 + t34092 - F::cast_from(0.62896184579208304136e-3_f64) * t34095 - t34100 + t34102 - F::cast_from(0.94344276868812456204e-2_f64) * t34105 + F::cast_from(0.94344276868812456204e-2_f64) * t34107 + F::cast_from(0.10718504529517434243e-2_f64) * t34111 + F::cast_from(0.42874018118069736972e-3_f64) * t34115;
    t34117
}
