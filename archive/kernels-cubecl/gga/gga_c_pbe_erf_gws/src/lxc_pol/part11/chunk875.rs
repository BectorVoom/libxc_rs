//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 875/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk875<F: Float>(t13671: F, t13672: F, t13674: F, t13675: F, t339: F, t338: F, t376: F, t2409: F, t3742: F, t8589: F, t3916: F, t3920: F) -> (F, F, F, F, F) {
    let t13677 = t13671 + t13672 + t13674 + t13675;
    let t13678 = t339 * t13677;
    let t13680 = t338 * t13678 * t376;
    let t13684 = t2409 * t8589 * t3742;
    let t13688 = t3916 * t3920;
    (t13677, t13678, t13680, t13684, t13688)
}
