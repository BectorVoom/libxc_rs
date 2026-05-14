//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 968/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk968<F: Float>(t17153: F, t5182: F, t2527: F, t4803: F, t1899: F, t1873: F, t1869: F, t1774: F, t3118: F, t7247: F, t1773: F, t1764: F, t7218: F, t4999: F, t7219: F, t25: F, t657: F) -> (F, F, F, F, F, F, F, F) {
    let t17154 = t5182 * t17153;
    let t17156 = t2527 * t4803;
    let t17157 = t1899 * t17156;
    let t17158 = t1873 * t17157;
    let t17159 = t1869 * t17158;
    let t17163 = t3118 * t1774;
    let t17164 = t17163 * t7247;
    let t17165 = t1773 * t17164;
    let t17169 = t1764 * t7218;
    let t17172 = t7219 * t4999;
    let t17182 = t25 * t657;
    (t17154, t17156, t17159, t17163, t17165, t17169, t17172, t17182)
}
