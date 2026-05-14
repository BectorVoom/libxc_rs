//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 831/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk831<F: Float>(t40086: F, t7487: F, t8362: F, t2001: F, t2281: F, t326: F, t333: F, t7720: F, t495: F, t515: F, t7230: F, t7231: F, t9109: F, t2868: F, t35566: F, t40050: F, t40055: F, t40057: F, t40060: F, t40063: F, t40068: F, t40073: F, t40076: F, t40082: F, t40085: F, t5055: F, t7527: F, t7530: F) -> (F,) {
    let t40087 = 0.19211284388664477842e-2 * t40086;
    let t40088 = t7487 * t8362;
    let t40089 = 0.19211284388664477842e-2 * t40088;
    let t40092 = t2001 * t326 * t2281 * t333;
    let t40093 = t7720 * t40092;
    let t40098 = t7230 * t7231 * t515 * t9109 * t495;
    let t40100 = 0.44903406381989282115e-1 * t40050 + 0.25538759935978703638e-4 * t40055 - 0.81823984962736025184e-1 * t40057 + 0.13637330827122670864e0 * t40060 - t40063 + 0.53205749866622299248e-5 * t40068 - 0.11971293719990017331e-4 * t40073 - t40076 + 0.35922725105591425692e0 * t5055 * t7527 + 0.23948483403727617128e0 * t2868 * t7530 - t35566 + 0.25538759935978703638e-4 * t40082 + t40085 + t40087 + t40089 + 0.25538759935978703638e-4 * t40093 - 0.1064114997332445985e-4 * t40098;
    (t40100,)
}
