//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1007/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1007(t15255: f64, t4566: f64, t11020: f64, t11086: f64, t15215: f64, t15219: f64, t15223: f64, t15224: f64, t15228: f64, t15232: f64, t15236: f64, t15241: f64, t15244: f64, t15249: f64, t15252: f64, t3514: f64, t5303: f64) -> (f64, f64) {
    let t15256 = t4566 * t15255;
    let t15257 = t11020 * t15256;
    let t15260 = -t11086 * t5303 / 81.0_f64 - t15215 - t15219 + t15223 + t3514 * t15224 / 432.0_f64 + 7.0_f64 / 1296.0_f64 * t3514 * t15228 + t3514 * t15232 / 108.0_f64 - t3514 * t15236 / 576.0_f64 - t3514 * t15241 / 144.0_f64 - t3514 * t15244 / 288.0_f64 + t3514 * t15249 / 288.0_f64 + t3514 * t15252 / 96.0_f64 - t3514 * t15257 / 216.0_f64;
    (t15256, t15260)
}
