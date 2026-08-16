//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1193/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1193(t122: f64, t2768: f64, t10673: f64, t10675: f64, t37419: f64, t37423: f64, t40278: f64, t40284: f64, t40288: f64, t40292: f64, t40294: f64, t40300: f64, t40302: f64, t40303: f64, t40305: f64, t40308: f64, t40313: f64, t40315: f64) -> f64 {
    let t40317 = t2768 * t122;
    let t40319 = t10673 * t10675 * t40317;
    let t40320 = 0.10248087766267884742e-3_f64 * t40319;
    let t40321 = t40278 + 0.29810146462873361018e-2_f64 * t37419 + 0.72042316457491791906e-3_f64 * t37423 + t40284 + t40288 - t40292 - 0.36021158228745895953e-3_f64 * t40294 - t40300 + t40302 - 0.19211284388664477842e-2_f64 * t40303 + 0.46116394948205481339e-3_f64 * t40305 + 0.36021158228745895953e-3_f64 * t40308 + t40313 - 0.43368970657079495312e-4_f64 * t40315 - t40320;
    t40321
}
