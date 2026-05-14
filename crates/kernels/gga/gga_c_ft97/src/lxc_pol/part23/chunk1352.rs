//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1352/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1352<F: Float>(t31600: F, t684: F, t99528: F, t99529: F, t1882: F, t31367: F, t1486: F, t31614: F, t681: F, t1476: F, t193: F, t19730: F, t6308: F, t852: F, t113420: F, t126949: F, t126954: F, t126958: F, t126963: F, t99780: F, t99782: F) -> (F, F, F, F, F) {
    let t126967 = t99528 * t99529 * t31600 * t684;
    let t126970 = t1882 * t31367;
    let t126971 = t126970 / 9.0;
    let t126973 = t1486 * t681 * t31614;
    let t126974 = 2.0 / 3.0 * t126973;
    let t126978 = t6308 * t193 * t852 * t1476 * t19730;
    let t126980 = t126949 / 4.0 + 2.0 * t126954 - 3.0 * t126958 + t99780 + t99782 - t126963 / 9.0 + t126967 / 3.0 - 8.0 / 9.0 * t113420 - t126971 - t126974 + t126978 / 4.0;
    (t126967, t126970, t126973, t126978, t126980)
}
