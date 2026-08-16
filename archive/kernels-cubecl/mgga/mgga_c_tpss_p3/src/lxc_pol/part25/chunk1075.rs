//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1075/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1075<F: Float>(t4892: F, t884: F, t1437: F, t3844: F, t4911: F, t4908: F, t2577: F, t4907: F, t3848: F, t4891: F, t8890: F, t4843: F, t8712: F) -> (F, F, F, F, F, F, F, F) {
    let t14804 = t4892 * t884;
    let t14807 = t1437 * t3844;
    let t14810 = t4911 * t884;
    let t14813 = t4908 * t884;
    let t14816 = t4907 * t2577;
    let t14817 = t14816 * t884;
    let t14820 = t3848 * t3844;
    let t14823 = t4891 * t8890;
    let t14824 = t14823 * t884;
    let t14827 = t4843 * t8712;
    (t14804, t14807, t14810, t14813, t14817, t14820, t14824, t14827)
}
