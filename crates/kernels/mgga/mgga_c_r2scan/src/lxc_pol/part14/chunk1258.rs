//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1258/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1258<F: Float>(t37455: F, t39074: F, t40388: F, t40391: F, t40411: F, t41316: F, t41319: F, t41322: F, t41329: F, t41332: F, t41335: F, t41339: F, t41342: F, t41346: F, t41350: F) -> F {
    let t42204 = t41316 - t41319 - F::cast_from(0.76845137554657911361e-2_f64) * t37455 - t41322 - F::cast_from(0.72042316457491791901e-3_f64) * t40388 - F::cast_from(0.1440846329149835838e-2_f64) * t40391 + t41329 - t41332 - t41335 - t41339 - t41342 - F::cast_from(0.1440846329149835838e-2_f64) * t40411 - t41346 + t41350 + t39074;
    t42204
}
