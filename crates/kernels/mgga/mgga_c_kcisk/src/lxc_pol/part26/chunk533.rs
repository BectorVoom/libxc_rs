//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 533/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk533<F: Float>(t397: F, t3979: F, t403: F, t396: F, t1323: F, t25: F, t1309: F, t1294: F, t1305: F, t1301: F, t172: F, t301: F, t342: F, t142: F, t416: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3981 = t397 * t3979 * t403;
    let t3983 = 0.11993859144118211475e-1 * t396 * t3981;
    let t3984 = t25 * t1323;
    let t3985 = t1309 * t3984;
    let t3996 = t1294 * t1305;
    let t4004 = t1301 * t1305;
    let t4007 = t342 * t172 * t301;
    let t4008 = 0.23744444444444444444e-1 * t4007;
    let t4009 = t142 * t416;
    (t3981, t3983, t3984, t3985, t3996, t4004, t4007, t4008, t4009)
}
