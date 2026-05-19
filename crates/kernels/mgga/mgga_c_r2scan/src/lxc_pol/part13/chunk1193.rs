//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1193/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1193<F: Float>(t122: F, t2768: F, t10673: F, t10675: F, t37419: F, t37423: F, t40278: F, t40284: F, t40288: F, t40292: F, t40294: F, t40300: F, t40302: F, t40303: F, t40305: F, t40308: F, t40313: F, t40315: F) -> F {
    let t40317 = t2768 * t122;
    let t40319 = t10673 * t10675 * t40317;
    let t40320 = F::cast_from(0.10248087766267884742e-3_f64) * t40319;
    let t40321 = t40278 + F::cast_from(0.29810146462873361018e-2_f64) * t37419 + F::cast_from(0.72042316457491791906e-3_f64) * t37423 + t40284 + t40288 - t40292 - F::cast_from(0.36021158228745895953e-3_f64) * t40294 - t40300 + t40302 - F::cast_from(0.19211284388664477842e-2_f64) * t40303 + F::cast_from(0.46116394948205481339e-3_f64) * t40305 + F::cast_from(0.36021158228745895953e-3_f64) * t40308 + t40313 - F::cast_from(0.43368970657079495312e-4_f64) * t40315 - t40320;
    t40321
}
