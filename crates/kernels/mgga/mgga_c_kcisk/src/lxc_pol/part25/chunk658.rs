//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 658/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk658<F: Float>(t2514: F, t3521: F, t4595: F, t708: F, t6759: F, t1648: F, t2364: F, t4604: F, t1824: F, t4609: F, t1876: F, t6764: F, t1877: F, t220: F, t1417: F, t2518: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6998 = t3521 * t2514;
    let t7000 = t4595 * t708;
    let t7001 = t7000 * t6759;
    let t7005 = t4604 * t2364 * t1648;
    let t7009 = t4609 * t2364 * t1824;
    let t7012 = t1876 * t708;
    let t7013 = t7012 * t6764;
    let t7017 = t1876 * t1877 * t220;
    let t7020 = t1417 * t2518;
    (t6998, t7000, t7001, t7005, t7009, t7012, t7013, t7017, t7020)
}
