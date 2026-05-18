//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1190/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1190<F: Float>(t1178: F, t371: F, t3737: F, t13830: F, t14617: F, t14657: F, t2409: F, t9897: F, t3965: F, t9818: F, t14121: F, t1105: F, t4182: F) -> (F, F, F, F, F, F, F, F) {
    let t15309 = t371 * t1178 * t3737;
    let t15310 = t13830 * t15309;
    let t15312 = t14657 * t14617;
    let t15314 = t2409 * t9897;
    let t15315 = t3965 * t15314;
    let t15317 = t2409 * t9818;
    let t15318 = t14121 * t15317;
    let t15320 = t4182 * t1105;
    (t15309, t15310, t15312, t15314, t15315, t15317, t15318, t15320)
}
