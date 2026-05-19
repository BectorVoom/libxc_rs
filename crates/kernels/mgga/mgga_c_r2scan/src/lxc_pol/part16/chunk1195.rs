//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1195/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1195<F: Float>(t37920: F, t39831: F, t39832: F, t39836: F, t39855: F, t41582: F, t43266: F, t43269: F, t43271: F, t43273: F, t43275: F, t43277: F) -> F {
    let t43279 = t39831 - F::cast_from(0.14282990759302185292e-1_f64) * t39832 + F::cast_from(0.27439371595564631661e-2_f64) * t43266 - t37920 - F::cast_from(0.65495539973149862688e-2_f64) * t43269 - F::cast_from(0.13099107994629972538e-1_f64) * t43271 - F::cast_from(0.52396431978519890152e-1_f64) * t43273 + F::cast_from(0.21831846657716620896e-2_f64) * t43275 + t39836 + F::cast_from(0.54878743191129263322e-2_f64) * t43277 - t41582 - t39855;
    t43279
}
