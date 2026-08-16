//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1322/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1322<F: Float>(t22336: F, t4083: F, t14883: F, t9270: F, t14959: F, t4414: F, t53545: F, t14185: F, t14958: F, t2408: F, t29751: F, t3060: F, t3212: F, t51505: F, t51507: F, t51509: F, t52191: F, t53531: F, t53537: F, t53542: F, t53549: F, t8754: F, t9283: F) -> F {
    let t55212 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t22336 * t4083;
    let t55218 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9270 * t14883;
    let t55228 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t4414 * t14959;
    let t55238 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53545;
    let t55240 = t55212 + t53531 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51505 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51507 - t53537 / F::cast_from(1536.0_f64) + t55218 - t2408 * t29751 * t14958 / F::cast_from(12.0_f64) + t53542 / F::cast_from(768.0_f64) - t2408 * t9283 * t52191 * t3060 / F::cast_from(12.0_f64) + t55228 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t51509 - t2408 * t9283 * t52191 * t3212 / F::cast_from(12.0_f64) - t2408 * t9283 * t14185 * t8754 / F::cast_from(12.0_f64) - t55238 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t53549;
    t55240
}
