//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1210/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1210<F: Float>(t2331: F, t328: F, t356: F, t3971: F, t3976: F, t15636: F, t3973: F, t13775: F, t13807: F, t371: F, t3970: F, t932: F) -> (F, F, F, F, F) {
    let t50948 = t356 * t328 * t2331 * t3971;
    let t50949 = t50948 * t3976;
    let t50956 = t3973 * t15636;
    let t50994 = t13807 * t13775;
    let t50998 = t3970 * t932 * t371;
    (t50948, t50949, t50956, t50994, t50998)
}
