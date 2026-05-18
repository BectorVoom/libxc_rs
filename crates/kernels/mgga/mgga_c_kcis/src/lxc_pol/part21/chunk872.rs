//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 872/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk872<F: Float>(t3212: F, t4566: F, t13410: F, t4554: F, t4823: F, t922: F, t9517: F, t3200: F, t4807: F, t9425: F, t4549: F, t1085: F, t2840: F) -> (F, F, F, F, F, F, F) {
    let t13416 = t4566 * t3212;
    let t13417 = t13410 * t13416;
    let t13418 = t4554 * t13417;
    let t13420 = t4823 * t922;
    let t13421 = t9517 * t13420;
    let t13422 = t3200 * t13421;
    let t13424 = t9425 * t4807;
    let t13426 = t9425 * t4549;
    let t13428 = t2840 * t1085;
    (t13416, t13418, t13420, t13422, t13424, t13426, t13428)
}
