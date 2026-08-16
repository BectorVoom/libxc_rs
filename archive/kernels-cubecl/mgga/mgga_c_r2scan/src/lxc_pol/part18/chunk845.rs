//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 845/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk845<F: Float>(t3124: F, t410: F, t229: F, t8590: F, t3034: F, t697: F, t41: F, t5384: F, t5392: F, t5401: F, t5405: F, t5409: F, t5413: F, t7739: F, t7743: F, t7745: F) -> F {
    let t8937 = t410 * t3124;
    let t8940 = t8590 * t229;
    let t8942 = t3034 * t697;
    let t8945 = t5384 - t5392 - t7739 - t5401 - t5405 + F::cast_from(0.42340699333333333333e-3_f64) * t5409 + F::cast_from(4.0_f64) * t8937 - t7743 - F::cast_from(0.21687162600603479684e-1_f64) * t7745 - t41 * t8940 + F::cast_from(0.65061487801810439052e-1_f64) * t8942 + F::cast_from(0.5848223622634646207e0_f64) * t5413;
    t8945
}
