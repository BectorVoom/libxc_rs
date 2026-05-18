//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 805/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk805<F: Float>(t10355: F, t10356: F, t2275: F, t606: F, t2258: F, t10326: F, t48: F, t58: F, t59: F, t2282: F, t60: F, t10199: F) -> (F, F, F, F, F, F, F) {
    let t10357 = t10355 * t10356;
    let t10360 = t2275 * t606;
    let t10361 = t10360 * t2258;
    let t10364 = t48 * t10326;
    let t10368 = F::new(1.0) / t59 / t58;
    let t10369 = t10368 * t10356;
    let t10372 = t2282 * t606;
    let t10373 = t10372 * t2258;
    let t10376 = t60 * t10326;
    let t10379 = F::new(1232.0) / F::new(27.0) * t10199;
    (t10357, t10361, t10364, t10369, t10373, t10376, t10379)
}
