//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 602/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk602<F: Float>(t1857: F, t970: F, t1856: F, t4648: F, t1835: F, t4640: F, t1836: F, t960: F, t706: F, t1843: F, t965: F, t1842: F) -> (F, F, F, F, F, F, F, F) {
    let t5142 = t970 * t1857;
    let t5144 = t1856 * t4648;
    let t5147 = t1835 * t4640;
    let t5150 = t960 * t1836;
    let t5152 = t1835 * t4648;
    let t5155 = t706 * t4640;
    let t5158 = t965 * t1843;
    let t5160 = t1842 * t4648;
    (t5142, t5144, t5147, t5150, t5152, t5155, t5158, t5160)
}
