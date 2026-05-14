//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1127/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1127<F: Float>(t12008: F, t13917: F, t13919: F, t15353: F, t9270: F, t14469: F, t53688: F, t3258: F, t51021: F, t56246: F, t814: F, t1105: F, t353: F, t4183: F, t4386: F, t1193: F, t3717: F) -> (F, F, F, F, F, F) {
    let t56265 = t13917 * t13919 * t12008;
    let t56267 = t9270 * t15353;
    let t56269 = t53688 * t14469;
    let t56276 = t13917 * t51021 * t3258 * t56246 * t814;
    let t56282 = t4386 * t353 * t4183 * t1105;
    let t56287 = t4386 * t353 * t1193 * t3717;
    (t56265, t56267, t56269, t56276, t56282, t56287)
}
