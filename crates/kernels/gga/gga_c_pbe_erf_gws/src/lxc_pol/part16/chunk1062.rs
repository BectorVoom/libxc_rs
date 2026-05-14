//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1062/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1062<F: Float>(t1176: F, t1180: F, t6589: F, t13987: F, t894: F, t13855: F, t13953: F, t13903: F, t3979: F, t3958: F, t6659: F, t332: F, t6158: F, t19911: F, t353: F, t859: F) -> (F, F, F, F, F, F, F) {
    let t51869 = t1176 * t6589 * t1180;
    let t51877 = t13987 * t894;
    let t51881 = t13953 * t13855;
    let t51896 = t3979 * t13903;
    let t51898 = t3958 * t6659;
    let t51916 = t6158 * t332;
    let t51919 = t859 * t353 * t19911;
    (t51869, t51877, t51881, t51896, t51898, t51916, t51919)
}
