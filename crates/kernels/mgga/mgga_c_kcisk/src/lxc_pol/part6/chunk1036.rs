//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1036/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1036<F: Float>(t31000: F, t3484: F, t3482: F, t13129: F, t2075: F, t7757: F, t2083: F, t3539: F, t7740: F, t13138: F, t7877: F, t2191: F, t3544: F) -> (F, F, F, F, F) {
    let t31001 = t3484 * t31000;
    let t31002 = t3482 * t31001;
    let t31009 = t13129 * t2075 * t7757;
    let t31013 = t3539 * t7740 * t2083;
    let t31017 = t13138 * t2075 * t7877;
    let t31021 = t3544 * t7740 * t2191;
    (t31002, t31009, t31013, t31017, t31021)
}
