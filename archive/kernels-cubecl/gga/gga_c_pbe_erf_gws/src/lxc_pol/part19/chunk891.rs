//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 891/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk891<F: Float>(t2376: F, t3886: F, t829: F, t830: F, t3912: F, t4396: F, t2358: F, t2246: F, t3903: F, t1109: F, t376: F, t810: F) -> (F, F, F, F, F, F, F) {
    let t9897 = t2376 * t3886;
    let t9899 = t829 * t830 * t9897;
    let t9902 = t3912 * t4396;
    let t9907 = t3912 * t2358;
    let t9912 = t2246 * t3903;
    let t9914 = t376 * t1109;
    let t9915 = t9914 * t810;
    (t9897, t9899, t9902, t9907, t9912, t9914, t9915)
}
