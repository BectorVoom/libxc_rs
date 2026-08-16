//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1480/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1480<F: Float>(t1504: F, t665: F, t8268: F, t31054: F, t658: F, t31058: F, t10199: F, t655: F, t2: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31259: F, t31261: F, t31264: F, t31268: F, t31271: F, t31274: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F) {
    let t31276 = t1504 * t665;
    let t31277 = t8268 * t31276;
    let t31280 = t31054 * t1504;
    let t31283 = t1504 * t658;
    let t31284 = t31058 * t31283;
    let t31287 = t10199 * t655;
    let t31288 = t8268 * t2;
    let t31291 = -t31026 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t31028 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31030 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31033 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t31259 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t31261 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t31264 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t31268 + t8258 * t31271 / F::cast_from(4.0_f64) + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31274 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t31277 + F::cast_from(25.0_f64) / F::cast_from(72.0_f64) * t8267 * t31280 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31284 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31287 * t31288;
    (t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31291)
}
