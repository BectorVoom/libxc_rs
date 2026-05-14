//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 663/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk663<F: Float>(t3912: F, t4396: F, t2358: F, t2246: F, t3903: F, t3744: F, t4414: F, t2366: F, t3916: F, t833: F, t3909: F, t840: F, t3342: F, t4757: F, t3351: F, t4767: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9902 = t3912 * t4396;
    let t9907 = t3912 * t2358;
    let t9912 = t2246 * t3903;
    let t9953 = t4414 * t3744;
    let t9955 = t3916 * t2366;
    let t9956 = t9955 * t833;
    let t9962 = t840 * t3909;
    let t9981 = t4757 * t3342;
    let t9993 = t4767 * t3351;
    (t9902, t9907, t9912, t9953, t9955, t9956, t9962, t9981, t9993)
}
