//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1050/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1050<F: Float>(t1175: F, t1410: F, t1460: F, t322: F, t1181: F, t3391: F, t6337: F, t1165: F, t1173: F, t1180: F, t1532: F, t16123: F, t16125: F, t16127: F, t16141: F, t16143: F, t16160: F, t20545: F, t3176: F, t372: F, t4261: F, t4262: F, t4298: F, t5275: F, t5606: F, t5922: F, t6218: F) -> (F, F) {
    let t21099 = t1175 * t1410;
    let t21118 = t1460 * t322;
    let t21121 = t3391 * t1181 * t6337 * t21118;
    let t21127 = -0.34299214494455789578e-2 * t1173 * t1165 * t5922 * t5275 + 0.34299214494455789578e-2 * t1173 * t1165 * t1532 * t21099 - 0.34299214494455789577e-2 * t16123 + 0.17149607247227894789e-2 * t16125 + 0.16006300097412701803e-1 * t16127 + 0.12004725073059526352e-1 * t16141 + 0.32012600194825403606e-1 * t16143 - t4261 * t4262 * t6218 * t372 / 12.0 - 0.13719685797782315831e-1 * t16160 + 0.10289764348336736874e-1 * t1180 * t1165 * t20545 * t3176 - 0.10289764348336736873e-1 * t21121 + 0.17149607247227894789e-2 * t1180 * t1165 * t4298 * t5606;
    (t21118, t21127)
}
