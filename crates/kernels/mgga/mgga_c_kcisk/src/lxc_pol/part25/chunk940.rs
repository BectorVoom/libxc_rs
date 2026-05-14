//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 940/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk940<F: Float>(t16617: F, t5203: F, t1800: F, t1869: F, t1894: F, t220: F, t5185: F, t5184: F, t15903: F, t2563: F, t4648: F, t5182: F, t4640: F, t6674: F, t2510: F, t3805: F) -> (F, F, F, F, F, F, F, F) {
    let t16618 = t5203 * t16617;
    let t16619 = t1800 * t16618;
    let t16620 = t1869 * t16619;
    let t16622 = t220 * t1894;
    let t16623 = t5185 * t16622;
    let t16624 = t5184 * t16623;
    let t16625 = t15903 * t16624;
    let t16627 = t2563 * t4648;
    let t16628 = t5184 * t16627;
    let t16629 = t5182 * t16628;
    let t16631 = t2563 * t4640;
    let t16632 = t5184 * t16631;
    let t16633 = t6674 * t16632;
    let t16640 = t3805 * t2510;
    (t16620, t16623, t16625, t16627, t16629, t16631, t16633, t16640)
}
