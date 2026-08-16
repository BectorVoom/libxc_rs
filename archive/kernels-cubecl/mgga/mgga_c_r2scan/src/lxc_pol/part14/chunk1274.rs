//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1274/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1274<F: Float>(t12366: F, t12367: F, t12368: F, t12220: F, t12223: F, t11453: F, t11457: F, t11460: F, t11463: F, t11467: F, t11471: F, t41147: F, t41148: F, t41149: F, t41150: F, t41193: F, t41237: F, t41277: F, t41323: F, t41809: F, t42356: F, t42360: F, t42364: F, t8: F) -> F {
    let t42369 = F::cast_from(2.0_f64) * t12366;
    let t42370 = F::cast_from(2.0_f64) * t12367;
    let t42371 = F::cast_from(2.0_f64) * t12368;
    let t42372 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t12220;
    let t42373 = t12223 / F::cast_from(2.0_f64);
    let t42374 = t11471 - t41147 + t41148 + t41149 + t41150 + t8 * (t41193 + t41237 + t41277 + t41323 + t41809 + t42356 + t42360 + t42364) + t42369 + t42370 + t11453 - t11457 - t11460 + t42371 - t42372 - t42373 - t11463 - t11467;
    t42374
}
