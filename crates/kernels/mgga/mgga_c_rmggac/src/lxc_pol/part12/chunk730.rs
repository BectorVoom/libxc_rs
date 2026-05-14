//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 730/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk730<F: Float>(t8610: F, t8623: F, t8627: F, t8633: F, t8637: F, t8643: F, t8647: F, t8651: F, t7545: F, t7550: F, t8653: F, t9381: F, t7670: F, t7674: F, t7679: F, t7681: F, t7684: F, t7686: F, t7689: F, t7693: F, t7698: F, t8673: F, t9412: F) -> (F, F, F) {
    let t38272 = 0.85129199786595678796e-5 * t8610;
    let t38274 = 0.13637330827122670864e-1 * t8623;
    let t38275 = 0.81823984962736025184e-1 * t8627;
    let t38276 = 0.13637330827122670864e0 * t8633;
    let t38277 = 0.27274661654245341728e-1 * t8637;
    let t38278 = 0.40911992481368012592e-1 * t8643;
    let t38279 = 0.81823984962736025184e-1 * t8647;
    let t38280 = 0.20455996240684006296e-1 * t8651;
    let t38282 = t38274 - t38275 + t38276 + t38277 + t38278 - t38279 - t38280 - 0.25538759935978703638e-4 * t8653 - t9381 + t7545 + t7550;
    let t38290 = -t7670 + 0.72732431077987577942e-1 * t8673 + t7674 - t7679 - t7681 - t7684 - t7686 - t7689 - t7693 - t7698 - t9412;
    (t38272, t38282, t38290)
}
