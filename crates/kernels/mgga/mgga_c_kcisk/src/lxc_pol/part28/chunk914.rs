//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 914/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk914<F: Float>(t17076: F, t2399: F, t4822: F, t10409: F, t6982: F, t1774: F, t3118: F, t7247: F, t1773: F, t1764: F, t7218: F, t4999: F, t7219: F, t25: F, t657: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17077 = 0.22109259259259259258e-2 * t17076;
    let t17078 = t2399 * t4822;
    let t17086 = t10409 * t6982;
    let t17087 = 0.14739506172839506172e-2 * t17086;
    let t17163 = t3118 * t1774;
    let t17164 = t17163 * t7247;
    let t17165 = t1773 * t17164;
    let t17169 = t1764 * t7218;
    let t17172 = t7219 * t4999;
    let t17182 = t25 * t657;
    (t17077, t17078, t17086, t17087, t17163, t17165, t17169, t17172, t17182)
}
