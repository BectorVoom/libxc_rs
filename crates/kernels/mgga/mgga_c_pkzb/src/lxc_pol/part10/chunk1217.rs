//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1217/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1217<F: Float>(t2380: F, t6475: F, t8463: F, t3174: F, t3176: F, t487: F, t68: F, t8269: F, t8281: F, t8277: F, t8435: F, t8437: F, t926: F, t1228: F, t300: F, t18657: F, t8376: F) -> (F, F, F, F, F, F, F, F) {
    let t22991 = t2380 * t6475 * t8463;
    let t23007 = t3174 * t487 * t3176;
    let t23010 = t3174 * t68 * t8269;
    let t23013 = t3174 * t68 * t8281;
    let t23020 = t3174 * t68 * t8277;
    let t23028 = t8435 * t926 * t8437;
    let t23054 = t300 * t1228;
    let t23061 = t2380 * t18657 * t8376;
    (t22991, t23007, t23010, t23013, t23020, t23028, t23054, t23061)
}
