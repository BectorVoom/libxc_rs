//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 844/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk844<F: Float>(t2441: F, t8939: F, t1899: F, t1873: F, t1869: F, t7283: F, t8845: F, t6697: F, t6856: F, t9137: F, t2394: F, t8590: F) -> (F, F, F, F, F, F) {
    let t28324 = t8939 * t2441;
    let t28325 = t1899 * t28324;
    let t28326 = t1873 * t28325;
    let t28327 = t1869 * t28326;
    let t28329 = t7283 * t8845;
    let t28332 = t6697 * t8939;
    let t28333 = t1873 * t28332;
    let t28334 = t1869 * t28333;
    let t28338 = t6856 * t9137;
    let t28341 = t8590 * t2394;
    (t28324, t28327, t28329, t28334, t28338, t28341)
}
