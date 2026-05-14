//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 745/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk745<F: Float>(t1309: F, t26470: F, t6157: F, t6196: F, t25: F, t8049: F, t425: F, t7764: F, t3521: F, t7858: F, t7846: F, t7757: F, t7870: F, t7862: F, t2059: F, t2083: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26471 = t1309 * t26470;
    let t26485 = t6157 * t6196;
    let t26489 = t25 * t8049;
    let t26490 = t1309 * t26489;
    let t26572 = t425 * t7764;
    let t26577 = t3521 * t7858;
    let t26579 = t3521 * t7846;
    let t26590 = t425 * t7757;
    let t26600 = t3521 * t7870;
    let t26602 = t3521 * t7862;
    let t26617 = t2059 * t2083;
    (t26471, t26485, t26490, t26572, t26577, t26579, t26590, t26600, t26602, t26617)
}
