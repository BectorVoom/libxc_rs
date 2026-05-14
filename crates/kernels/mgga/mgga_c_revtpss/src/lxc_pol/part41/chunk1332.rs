//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1332/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1332<F: Float>(t1513: F, t31039: F, t658: F, t8268: F, t4287: F, t8259: F, t31032: F, t8358: F, t1504: F, t665: F, t31054: F, t31058: F, t10199: F, t655: F, t2: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31259: F, t31261: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31264 = t31039 * t1513;
    let t31267 = t1513 * t658;
    let t31268 = t8268 * t31267;
    let t31271 = t8259 * t4287;
    let t31274 = t31032 * t8358;
    let t31276 = t1504 * t665;
    let t31277 = t8268 * t31276;
    let t31280 = t31054 * t1504;
    let t31283 = t1504 * t658;
    let t31284 = t31058 * t31283;
    let t31287 = t10199 * t655;
    let t31288 = t8268 * t2;
    let t31291 = -t31026 - 2.0 / 3.0 * t31028 - 5.0 / 9.0 * t31030 + 5.0 / 9.0 * t31033 - 2.0 / 3.0 * t31259 - 3.0 / 4.0 * t31035 * t31261 - 5.0 / 12.0 * t8258 * t31264 + 5.0 / 12.0 * t8258 * t31268 + t8258 * t31271 / 4.0 + 5.0 / 9.0 * t31274 + 5.0 / 12.0 * t8258 * t31277 + 25.0 / 72.0 * t8267 * t31280 - 5.0 / 36.0 * t8267 * t31284 - 5.0 / 24.0 * t31287 * t31288;
    (t31264, t31268, t31271, t31274, t31276, t31277, t31280, t31283, t31284, t31287, t31288, t31291)
}
