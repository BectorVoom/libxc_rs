//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 936/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk936<F: Float>(t16103: F, t16567: F, t673: F, t716: F, t720: F, t415: F, t1636: F, t6702: F, t10426: F, t5182: F, t5191: F, t5203: F, t6707: F, t5199: F, t6719: F, t1799: F) -> (F, F, F, F, F, F, F, F) {
    let t16568 = t16103 + t16567;
    let t16569 = t673 * t16568;
    let t16570 = t16569 * t716;
    let t16571 = t16570 * t720;
    let t16572 = t415 * t16571;
    let t16576 = t6702 * t1636;
    let t16577 = t10426 * t16576;
    let t16578 = t5182 * t16577;
    let t16580 = t5191 * t5203;
    let t16581 = t6707 * t1636;
    let t16582 = t16580 * t16581;
    let t16583 = t5182 * t16582;
    let t16585 = t6719 * t5199;
    let t16586 = t1799 * t16585;
    (t16568, t16569, t16572, t16576, t16578, t16581, t16583, t16586)
}
