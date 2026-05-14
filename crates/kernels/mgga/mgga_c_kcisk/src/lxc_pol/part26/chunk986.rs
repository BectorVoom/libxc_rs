//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 986/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk986<F: Float>(t3521: F, t7870: F, t7862: F, t1354: F, t7877: F, t1175: F, t3564: F, t1364: F, t8108: F, t5953: F, t19155: F, t5944: F, t3558: F, t442: F, t2059: F, t2083: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26600 = t3521 * t7870;
    let t26602 = t3521 * t7862;
    let t26604 = t1354 * t7877;
    let t26605 = t26604 * t1175;
    let t26606 = t3564 * t26605;
    let t26609 = t8108 * t1364;
    let t26610 = t5953 * t26609;
    let t26613 = t19155 * t5944;
    let t26616 = t3558 * t442;
    let t26617 = t2059 * t2083;
    (t26600, t26602, t26605, t26606, t26609, t26610, t26613, t26616, t26617)
}
