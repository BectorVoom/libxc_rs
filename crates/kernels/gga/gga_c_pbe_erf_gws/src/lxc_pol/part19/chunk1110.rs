//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1110/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1110<F: Float>(t53625: F, t1205: F, t20173: F, t53645: F, t14918: F, t2367: F, t53725: F, t53727: F, t53729: F, t53806: F, t14902: F, t9270: F, t14928: F, t840: F, t53873: F, t15018: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t55290 = 7.0 / 576.0 * t53625;
    let t55297 = t20173 * t1205;
    let t55311 = 7.0 / 72.0 * t53645;
    let t55315 = 7.0 / 144.0 * t2367 * t14918;
    let t55344 = 7.0 / 72.0 * t53725;
    let t55345 = 7.0 / 1152.0 * t53727;
    let t55351 = 7.0 / 576.0 * t53729;
    let t55375 = 7.0 / 12.0 * t53806;
    let t55382 = 7.0 / 72.0 * t9270 * t14902;
    let t55385 = 7.0 / 144.0 * t840 * t14928;
    let t55403 = 7.0 / 576.0 * t53873;
    let t55420 = 7.0 / 144.0 * t840 * t15018;
    (t55290, t55297, t55311, t55315, t55344, t55345, t55351, t55375, t55382, t55385, t55403, t55420)
}
