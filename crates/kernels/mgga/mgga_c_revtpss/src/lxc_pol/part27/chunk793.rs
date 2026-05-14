//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 793/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk793<F: Float>(t10355: F, t10356: F, t2275: F, t606: F, t2258: F, t10326: F, t48: F, t58: F, t59: F, t2282: F, t60: F, t10199: F, t10345: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> (F, F) {
    let t10357 = t10355 * t10356;
    let t10360 = t2275 * t606;
    let t10361 = t10360 * t2258;
    let t10364 = t48 * t10326;
    let t10368 = 1.0 / t59 / t58;
    let t10369 = t10368 * t10356;
    let t10372 = t2282 * t606;
    let t10373 = t10372 * t2258;
    let t10376 = t60 * t10326;
    let t10379 = 1232.0 / 27.0 * t10199;
    let t10380 = -1232.0 / 27.0 * t10345 * t49 + 220.0 / 9.0 * t2270 * t617 - 20.0 / 9.0 * t614 * t2276 - 20.0 / 3.0 * t614 * t2279 - 5.0 / 108.0 * t44 * t10357 + 5.0 / 6.0 * t44 * t10361 + 5.0 / 6.0 * t44 * t10364 + 5.0 / 108.0 * t56 * t10369 + 5.0 / 6.0 * t56 * t10373 - 5.0 / 6.0 * t56 * t10376 + t10379;
    (t10368, t10380)
}
