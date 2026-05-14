//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 897/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk897<F: Float>(t11465: F, t3269: F, t11008: F, t11377: F, t11378: F, t11379: F, t11380: F, t11453: F, t11454: F, t11457: F, t11460: F, t11463: F, t10643: F, t10653: F, t10660: F, t11185: F, t11188: F, t11192: F, t11195: F, t11198: F, t11202: F, t11357: F, t11369: F, t11376: F) -> (F, F) {
    let t11466 = t3269 * t11465;
    let t11467 = 5.0 / 8.0 * t11466;
    let t11468 = t11377 - t11378 + t11379 + t11380 - 0.162600798888400151e-2 * t11008 - t11453 - t11454 + t11457 + t11460 + t11463 + t11467;
    let t11470 = t11185 + 0.30487649791575028312e-3 * t10643 - t11188 - t11192 + 0.1440846329149835838e-2 * t10653 - t11195 - t11357 - 0.60975299583150056624e-3 * t10660 - t11198 - t11202 + t11369 + t11376 + t11468;
    (t11467, t11470)
}
