//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1050/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1050<F: Float>(t15278: F, t3972: F, t13780: F, t3742: F, t3990: F, t13859: F, t13942: F, t2080: F, t3803: F, t833: F, t1178: F, t371: F, t3722: F, t1177: F, t3737: F, t13830: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15279 = t3972 * t15278;
    let t15282 = t3990 * t13780 * t3742;
    let t15283 = t13859 * t15282;
    let t15288 = t2080 * t3803 * t13942;
    let t15289 = t15288 * t833;
    let t15296 = t371 * t1178 * t3722;
    let t15297 = t1177 * t15296;
    let t15309 = t371 * t1178 * t3737;
    let t15310 = t13830 * t15309;
    (t15279, t15282, t15283, t15288, t15289, t15296, t15297, t15309, t15310)
}
