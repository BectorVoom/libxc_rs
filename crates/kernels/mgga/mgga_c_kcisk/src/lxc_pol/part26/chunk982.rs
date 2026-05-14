//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 982/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk982<F: Float>(t1440: F, t7706: F, t5625: F, t3796: F, t3482: F, t25406: F, t3484: F, t5634: F, t5633: F, t1163: F, t8077: F, t13377: F, t25469: F, t5895: F, t2075: F, t3539: F, t5684: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26503 = t7706 * t1440;
    let t26504 = t5625 * t26503;
    let t26505 = t3796 * t26504;
    let t26506 = t3482 * t26505;
    let t26508 = t5625 * t25406;
    let t26509 = t3484 * t26508;
    let t26510 = t3482 * t26509;
    let t26512 = t5634 * t25406;
    let t26513 = t3484 * t26512;
    let t26514 = t5633 * t26513;
    let t26516 = t8077 * t1163;
    let t26517 = t13377 * t26516;
    let t26518 = t3482 * t26517;
    let t26520 = t5895 * t25469;
    let t26524 = t3539 * t2075 * t5684;
    (t26503, t26504, t26506, t26508, t26510, t26512, t26514, t26516, t26518, t26520, t26524)
}
