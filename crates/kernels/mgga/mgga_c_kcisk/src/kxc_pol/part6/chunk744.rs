//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 744/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk744<F: Float>(t1248: F, t13603: F, t7736: F, t3979: F, t7740: F, t7744: F, t4126: F, t7993: F, t45: F, t7970: F, t4083: F, t7959: F, t1305: F, t8022: F, t25: F, t8055: F) -> (F, F, F, F, F, F, F, F) {
    let t26176 = t1248 * t13603 * t7736;
    let t26179 = t1248 * t3979 * t7740;
    let t26198 = t1248 * t3979 * t7744;
    let t26302 = t4126 * t7993;
    let t26341 = t45 * t7970;
    let t26344 = t7959 * t4083;
    let t26430 = t8022 * t1305;
    let t26470 = t25 * t8055;
    (t26176, t26179, t26198, t26302, t26341, t26344, t26430, t26470)
}
