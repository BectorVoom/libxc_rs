//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 913/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk913<F: Float>(t15255: F, t4566: F, t11020: F, t11086: F, t15215: F, t15219: F, t15223: F, t15224: F, t15228: F, t15232: F, t15236: F, t15241: F, t15244: F, t15249: F, t15252: F, t3514: F, t5303: F) -> (F, F) {
    let t15256 = t4566 * t15255;
    let t15257 = t11020 * t15256;
    let t15260 = -t11086 * t5303 / 81.0 - t15215 - t15219 + t15223 + t3514 * t15224 / 432.0 + 7.0 / 1296.0 * t3514 * t15228 + t3514 * t15232 / 108.0 - t3514 * t15236 / 576.0 - t3514 * t15241 / 144.0 - t3514 * t15244 / 288.0 + t3514 * t15249 / 288.0 + t3514 * t15252 / 96.0 - t3514 * t15257 / 216.0;
    (t15256, t15260)
}
