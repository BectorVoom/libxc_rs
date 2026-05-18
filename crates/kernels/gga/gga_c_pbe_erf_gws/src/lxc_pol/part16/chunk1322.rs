//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1322/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1322<F: Float>(t22336: F, t4083: F, t14883: F, t9270: F, t14959: F, t4414: F, t53545: F, t14185: F, t14958: F, t2408: F, t29751: F, t3060: F, t3212: F, t51505: F, t51507: F, t51509: F, t52191: F, t53531: F, t53537: F, t53542: F, t53549: F, t8754: F, t9283: F) -> F {
    let t55212 = F::new(7.0) / F::new(144.0) * t22336 * t4083;
    let t55218 = F::new(7.0) / F::new(24.0) * t9270 * t14883;
    let t55228 = F::new(7.0) / F::new(36.0) * t4414 * t14959;
    let t55238 = F::new(7.0) / F::new(288.0) * t53545;
    let t55240 = t55212 + t53531 / F::new(12.0) - F::new(7.0) / F::new(144.0) * t51505 - F::new(7.0) / F::new(1152.0) * t51507 - t53537 / F::new(1536.0) + t55218 - t2408 * t29751 * t14958 / F::new(12.0) + t53542 / F::new(768.0) - t2408 * t9283 * t52191 * t3060 / F::new(12.0) + t55228 - F::new(119.0) / F::new(3456.0) * t51509 - t2408 * t9283 * t52191 * t3212 / F::new(12.0) - t2408 * t9283 * t14185 * t8754 / F::new(12.0) - t55238 - F::new(5.0) / F::new(384.0) * t53549;
    t55240
}
