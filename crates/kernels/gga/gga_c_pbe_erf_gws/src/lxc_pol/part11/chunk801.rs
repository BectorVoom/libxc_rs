//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 801/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk801<F: Float>(t13677: F, t339: F, t338: F, t376: F, t2409: F, t3742: F, t8589: F, t3916: F, t3920: F, t1162: F, t3907: F, t1115: F, t12111: F, t12195: F, t12199: F, t12223: F, t12246: F, t12253: F, t13641: F, t13645: F, t13650: F, t13656: F, t13662: F, t2401: F, t2408: F, t2503: F, t335: F, t3921: F, t833: F, t844: F, t8659: F, t9820: F, t9899: F) -> (F, F, F, F, F, F) {
    let t13678 = t339 * t13677;
    let t13680 = t338 * t13678 * t376;
    let t13684 = t2409 * t8589 * t3742;
    let t13688 = t3916 * t3920;
    let t13695 = t338 * t3907 * t1162;
    let t13698 = -t844 * t13641 / 48.0 - t844 * t13645 / 16.0 - t2408 * t13650 / 8.0 - 7.0 / 48.0 * t12195 - 7.0 / 96.0 * t12199 + 3.0 / 16.0 * t2401 * t13656 + 7.0 / 48.0 * t12223 + t8659 * t13662 / 48.0 + t1115 * t12111 / 16.0 - t1115 * t9899 / 32.0 + 3.0 / 16.0 * t1115 * t9820 + t335 * t13680 / 96.0 + t2408 * t13684 / 8.0 + 7.0 / 96.0 * t12246 + t13688 * t833 / 48.0 + t3921 * t2503 / 32.0 + 7.0 / 48.0 * t12253 - t335 * t13695 / 32.0;
    (t13678, t13680, t13684, t13688, t13695, t13698)
}
