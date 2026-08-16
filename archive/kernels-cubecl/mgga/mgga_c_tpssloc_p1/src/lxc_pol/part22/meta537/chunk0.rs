//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2016/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2016<F: Float>(t2511: F, t39377: F, t39378: F, t1294: F, t2504: F, t2368: F, t746: F, t268: F, t676: F, t9478: F, t9482: F) -> (F, F, F, F, F, F, F) {
    let t39380 = t2511 * t2511;
    let t39381 = F::cast_from(1.0_f64) / t39380;
    let t39382 = t39377 * t39378 * t39381;
    let t39384 = F::cast_from(0.91082604192152556044e5_f64) * t1294 * t39382;
    let t39389 = t2504 * t2504;
    let t39391 = t2368 * t39389 * t746;
    let t39393 = F::cast_from(0.35089341735807877242e1_f64) * t1294 * t39391;
    let t39397 = F::cast_from(0.3684616320282908548e2_f64) * t268 * t676 * t9478 * t9482;
    (t39381, t39382, t39384, t39389, t39391, t39393, t39397)
}
