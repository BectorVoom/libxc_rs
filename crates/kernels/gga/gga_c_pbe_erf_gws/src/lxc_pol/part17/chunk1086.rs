//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1086/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1086<F: Float>(t13888: F, t2410: F, t9283: F, t1176: F, t2333: F, t1180: F, t2397: F, t3952: F, t1178: F, t2353: F, t371: F, t1177: F) -> (F, F, F, F, F, F, F) {
    let t13889 = t13888 * t2410;
    let t13890 = t9283 * t13889;
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13895 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t13894;
    let t13896 = t3952 * t2397;
    let t13899 = t371 * t1178 * t2353;
    let t13900 = t1177 * t13899;
    (t13889, t13890, t13893, t13895, t13896, t13899, t13900)
}
