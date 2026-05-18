//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1189/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1189<F: Float>(t13942: F, t2080: F, t3803: F, t833: F, t1144: F, t338: F, t4183: F, t1178: F, t371: F, t3722: F, t1177: F, t1193: F, t3907: F) -> (F, F, F, F, F, F) {
    let t15288 = t2080 * t3803 * t13942;
    let t15289 = t15288 * t833;
    let t15292 = t338 * t1144 * t4183;
    let t15296 = t371 * t1178 * t3722;
    let t15297 = t1177 * t15296;
    let t15300 = t338 * t3907 * t1193;
    (t15288, t15289, t15292, t15296, t15297, t15300)
}
