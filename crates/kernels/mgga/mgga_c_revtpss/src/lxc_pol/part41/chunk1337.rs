//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1337/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1337<F: Float>(t114: F, t2178: F, t6765: F, t6934: F, t5891: F, t8259: F, t1504: F, t1513: F, t8268: F, t5915: F, t31058: F, t5895: F, t5823: F, t31026: F, t31035: F, t31259: F, t31274: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t31518 = t6765 * t2178;
    let t31533 = t2178 * t6934;
    let t31538 = t8259 * t5891;
    let t31541 = t1513 * t1504;
    let t31542 = t8268 * t31541;
    let t31545 = t8259 * t5915;
    let t31548 = t31058 * t5895;
    let t31551 = t8268 * t5823;
    let t31555 = piecewise3(t115, 0.0, -t31026 - 4.0 / 3.0 * t31259 + 10.0 / 9.0 * t31274 - 3.0 / 4.0 * t31035 * t31538 + 5.0 / 6.0 * t8258 * t31542 + t8258 * t31545 / 4.0 - 5.0 / 36.0 * t8267 * t31548 - 5.0 / 24.0 * t8267 * t31551);
    (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551, t31555)
}
