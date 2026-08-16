//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 844/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk844(t3124: f64, t410: f64, t229: f64, t8590: f64, t3034: f64, t697: f64, t41: f64, t5384: f64, t5392: f64, t5401: f64, t5405: f64, t5409: f64, t5413: f64, t7739: f64, t7743: f64, t7745: f64) -> f64 {
    let t8937 = t410 * t3124;
    let t8940 = t8590 * t229;
    let t8942 = t3034 * t697;
    let t8945 = t5384 - t5392 - t7739 - t5401 - t5405 + 0.42340699333333333333e-3_f64 * t5409 + 4.0_f64 * t8937 - t7743 - 0.21687162600603479684e-1_f64 * t7745 - t41 * t8940 + 0.65061487801810439052e-1_f64 * t8942 + 0.5848223622634646207e0_f64 * t5413;
    t8945
}
