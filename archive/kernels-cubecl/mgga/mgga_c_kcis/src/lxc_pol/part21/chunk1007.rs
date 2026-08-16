//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1007/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1007<F: Float>(t15255: F, t4566: F, t11020: F, t11086: F, t15215: F, t15219: F, t15223: F, t15224: F, t15228: F, t15232: F, t15236: F, t15241: F, t15244: F, t15249: F, t15252: F, t3514: F, t5303: F) -> (F, F) {
    let t15256 = t4566 * t15255;
    let t15257 = t11020 * t15256;
    let t15260 = -t11086 * t5303 / F::cast_from(81.0_f64) - t15215 - t15219 + t15223 + t3514 * t15224 / F::cast_from(432.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1296.0_f64) * t3514 * t15228 + t3514 * t15232 / F::cast_from(108.0_f64) - t3514 * t15236 / F::cast_from(576.0_f64) - t3514 * t15241 / F::cast_from(144.0_f64) - t3514 * t15244 / F::cast_from(288.0_f64) + t3514 * t15249 / F::cast_from(288.0_f64) + t3514 * t15252 / F::cast_from(96.0_f64) - t3514 * t15257 / F::cast_from(216.0_f64);
    (t15256, t15260)
}
