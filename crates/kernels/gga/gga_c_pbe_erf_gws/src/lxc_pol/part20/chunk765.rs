//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 765/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk765<F: Float>(t219: F, t7209: F, t1621: F, t1791: F, t21: F, t5589: F, t2719: F, t1041: F, t1251: F, t1028: F, t1243: F, t4: F, t4573: F, t2706: F, t2698: F, t395: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7210 = t7209 * t219;
    let t7216 = t1621 * t1791;
    let t7236 = t21 * t5589;
    let t7237 = t7236 * t2719;
    let t7239 = t1251 * t1041;
    let t7269 = t1243 * t1028;
    let t7271 = t4 * t4573;
    let t7272 = t7271 * t2706;
    let t7278 = t395 * t2698;
    (t7210, t7216, t7236, t7237, t7239, t7269, t7271, t7272, t7278)
}
