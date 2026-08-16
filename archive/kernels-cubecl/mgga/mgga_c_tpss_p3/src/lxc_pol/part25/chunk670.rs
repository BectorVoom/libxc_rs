//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 670/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk670<F: Float>(t4245: F, t450: F, t1112: F, t242: F, t1098: F, t1103: F, t1111: F, t3027: F, t3029: F, t3052: F, t3067: F, t4210: F, t4212: F, t4217: F, t4220: F, t4224: F, t4228: F, t4234: F, t4239: F, t4242: F) -> (F, F, F) {
    let t4246 = t4245 * t450;
    let t4247 = t1112 * t4246;
    let t4248 = t242 * t4247;
    let t4251 = -t4210 / F::cast_from(108.0_f64) + t4212 * t1103 / F::cast_from(108.0_f64) - t3027 - t3029 / F::cast_from(864.0_f64) - t4217 / F::cast_from(864.0_f64) + t1098 * t4220 / F::cast_from(216.0_f64) - t1098 * t4224 / F::cast_from(144.0_f64) - t1098 * t4228 / F::cast_from(288.0_f64) + t3052 * t4234 / F::cast_from(1536.0_f64) + t4239 / F::cast_from(4608.0_f64) - t3067 * t4242 / F::cast_from(4608.0_f64) + t1111 * t4248 / F::cast_from(3072.0_f64);
    (t4246, t4248, t4251)
}
