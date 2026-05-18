//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1095/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1095<F: Float>(t3316: F, t5047: F, t7748: F, t3424: F, t377: F, t3362: F, t374: F, t982: F, t7755: F, t1096: F, t3432: F, t386: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t26917 = t5047 * t3316;
    let t26918 = t7748 * t26917;
    let t26920 = t3424 * t377;
    let t26922 = t374 * t3362;
    let t26924 = t374 * t982;
    let t26925 = t26924 * t7755;
    let t26927 = t1096 * t3432;
    let t26929 = sigma0 * t386;
    (t26917, t26918, t26920, t26922, t26924, t26925, t26927, t26929)
}
