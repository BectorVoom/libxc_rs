//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1303/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1303<F: Float>(t13808: F, t14584: F, t4130: F, t51650: F, t2409: F, t26880: F, t3959: F, t13893: F, t4150: F, t14596: F, t3965: F, t9299: F) -> (F, F, F, F, F, F) {
    let t54716 = t13808 * t14584;
    let t54719 = t51650 * t4130;
    let t54722 = t3959 * t2409 * t26880;
    let t54724 = t13893 * t4150;
    let t54730 = t13808 * t14596;
    let t54734 = t3965 * t9299;
    (t54716, t54719, t54722, t54724, t54730, t54734)
}
