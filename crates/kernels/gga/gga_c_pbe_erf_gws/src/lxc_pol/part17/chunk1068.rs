//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1068/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1068<F: Float>(t13903: F, t3979: F, t3958: F, t6659: F, t14109: F, t840: F, t332: F, t6158: F, t19911: F, t353: F, t859: F, t4408: F, t13869: F, t13972: F, t13881: F, t4052: F, t6781: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51896 = t3979 * t13903;
    let t51898 = t3958 * t6659;
    let t51906 = t840 * t14109;
    let t51916 = t6158 * t332;
    let t51919 = t859 * t353 * t19911;
    let t51922 = t4408 * t332;
    let t51928 = t13972 * t13869;
    let t51930 = t840 * t13881;
    let t51945 = t6781 * t4052;
    (t51896, t51898, t51906, t51916, t51919, t51922, t51928, t51930, t51945)
}
