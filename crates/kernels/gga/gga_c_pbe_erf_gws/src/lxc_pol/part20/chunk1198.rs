//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1198/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1198<F: Float>(t1113: F, t1161: F, t13781: F, t2271: F, t3972: F, t1109: F, t3975: F, t4408: F, t2409: F, t35910: F, t3965: F, t13808: F, t15146: F, t15191: F, t50994: F, t3306: F, t824: F) -> (F, F, F, F, F, F) {
    let t57626 = t3972 * t13781 * t1113 * t2271 * t1161;
    let t57635 = t3972 * t3975 * t1113 * t4408 * t1109;
    let t57639 = t3965 * t2409 * t35910;
    let t57641 = t13808 * t15146;
    let t57643 = t50994 * t15191;
    let t57648 = t3972 * t13781 * t1113 * t824 * t3306;
    (t57626, t57635, t57639, t57641, t57643, t57648)
}
