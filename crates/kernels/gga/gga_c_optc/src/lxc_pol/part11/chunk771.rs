//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 771/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk771<F: Float>(t1502: F, t7274: F, t1162: F, t1179: F, t12489: F, t4434: F, t7448: F, t140: F, t1514: F, t2665: F, t3183: F, t3101: F) -> (F, F, F, F, F, F) {
    let t12726 = t7274 * t1502;
    let t12727 = t1162 * t12726;
    let t12729 = t1179 * t12489;
    let t12741 = t4434 * t7448;
    let t12798 = t1514 * t2665 * t140;
    let t12799 = t3183 * t12798;
    let t12802 = t3101 * t12798;
    (t12726, t12727, t12729, t12741, t12799, t12802)
}
