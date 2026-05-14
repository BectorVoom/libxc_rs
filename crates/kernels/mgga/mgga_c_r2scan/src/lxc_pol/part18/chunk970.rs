//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 970/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk970<F: Float>(t10684: F, t38355: F, t10648: F, t10958: F, t10971: F, t10962: F, t11477: F, t11481: F, t11484: F, t11488: F, t11491: F, t11494: F, t11499: F, t11503: F, t11507: F, t11511: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38356 = t38355 * t10684;
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38363 = 0.45731474687362542471e-3 * t38362;
    let t39149 = 3.0 / 2.0 * t11477;
    let t39150 = t11481 / 2.0;
    let t39151 = t11484 / 2.0;
    let t39152 = 15.0 / 8.0 * t11488;
    let t39153 = 3.0 / 2.0 * t11491;
    let t39154 = t11494 / 2.0;
    let t39155 = 3.0 / 2.0 * t11499;
    let t39156 = 3.0 / 2.0 * t11503;
    let t39157 = 3.0 / 2.0 * t11507;
    let t39159 = 3.0 * t11511;
    (t38356, t38359, t38363, t39149, t39150, t39151, t39152, t39153, t39154, t39155, t39156, t39157, t39159)
}
