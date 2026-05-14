//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 646/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk646<F: Float>(t1750: F, t663: F, t172: F, t1773: F, t184: F, t564: F, t5324: F, t5326: F, t5328: F, t5330: F, t5332: F, t5337: F, t5339: F, t5341: F, t5345: F, t5348: F, t5350: F, t5354: F, t5356: F, t5359: F, t5375: F) -> (F, F, F, F, F) {
    let t5377 = 2.0 / 5.0 * t1750 * t663;
    let t5378 = t172 * t1773;
    let t5379 = t5378 * t184;
    let t5381 = 4.0 / 5.0 * t5379 * t564;
    let t5382 = -t5324 - t5326 - t5328 + t5330 + t5332 + t5337 + t5339 + t5341 + t5345 + t5348 + t5350 + t5354 - t5356 + t5359 + t5375 - t5377 + t5381;
    (t5377, t5378, t5379, t5381, t5382)
}
