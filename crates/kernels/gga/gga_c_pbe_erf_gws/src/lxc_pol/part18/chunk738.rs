//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 738/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk738<F: Float>(t1235: F, t125: F, t128: F, t2: F, t39: F, t1576: F, t510: F, t512: F, t131: F, t120: F, t133: F, t1365: F, t5783: F, t1378: F, t1971: F, t5701: F) -> (F, F, F, F, F, F) {
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = 0.32645333333333333334e0 * t5832 * t5833 * t39;
    let t5847 = t510 * t1576;
    let t5852 = t512 * t512;
    let t5853 = 1.0 / t5852;
    let t5854 = t131 * t5853;
    let t5863 = 0.89405814814814814813e0 * t133 * t1365 * t120;
    let t5864 = t133 * t5783;
    let t5891 = t5701 * t1378 * t1971;
    (t5836, t5847, t5854, t5863, t5864, t5891)
}
