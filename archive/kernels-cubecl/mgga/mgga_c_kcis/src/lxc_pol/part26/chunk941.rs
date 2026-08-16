//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 941/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk941<F: Float>(t21860: F, t4160: F, t21858: F, t5426: F, t15865: F, t5661: F, t1363: F, t7028: F, t3738: F, t7037: F, t4153: F, t11913: F, t6905: F) -> (F, F, F, F, F, F) {
    let t21861 = t4160 * t21860;
    let t21863 = t5426 * t21858;
    let t21864 = t15865 * t21863;
    let t21865 = t5661 * t21864;
    let t21868 = t7028 * t1363;
    let t21871 = t3738 * t7037;
    let t21872 = t4153 * t21871;
    let t21874 = t11913 * t6905;
    (t21861, t21863, t21865, t21868, t21872, t21874)
}
