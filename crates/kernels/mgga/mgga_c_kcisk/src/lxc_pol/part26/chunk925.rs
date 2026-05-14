//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 925/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk925<F: Float>(t25454: F, t25506: F, t25528: F, t25552: F, t1346: F, t8111: F, t1391: F, t8099: F, t443: F, t8105: F, t8108: F, t25312: F, t425: F, t1175: F, t7757: F, t2083: F, t5684: F) -> (F, F, F, F, F, F, F, F) {
    let t25554 = t25454 + t25506 + t25528 + t25552;
    let t25557 = t1346 * t8111;
    let t25559 = t1391 * t8099;
    let t25561 = t443 * t8105;
    let t25563 = t1346 * t8108;
    let t25565 = t425 * t25312;
    let t25568 = t7757 * t1175;
    let t25573 = t2083 * t5684;
    (t25554, t25557, t25559, t25561, t25563, t25565, t25568, t25573)
}
