//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 978/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk978<F: Float>(t11458: F, t3262: F, t10622: F, t3275: F, t3472: F, t11002: F, t1115: F, t792: F, t3269: F, t11008: F, t11377: F, t11378: F, t11379: F, t11380: F, t11453: F, t11454: F, t11457: F) -> (F, F, F, F, F) {
    let t11459 = t3262 * t11458;
    let t11460 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11459;
    let t11462 = t3275 * t3472 * t10622;
    let t11463 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11462;
    let t11465 = t11002 * t1115 * t792;
    let t11466 = t3269 * t11465;
    let t11467 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t11466;
    let t11468 = t11377 - t11378 + t11379 + t11380 - F::cast_from(0.162600798888400151e-2_f64) * t11008 - t11453 - t11454 + t11457 + t11460 + t11463 + t11467;
    (t11460, t11463, t11465, t11467, t11468)
}
