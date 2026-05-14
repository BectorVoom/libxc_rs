//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1025/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1025<F: Float>(t12133: F, t498: F, t16848: F, t12159: F, t613: F, t1380: F, t1938: F, t12230: F, t1924: F, t4479: F, t6220: F, t1505: F, t17306: F, t1628: F, t18266: F, t1610: F, t6183: F) -> (F, F, F, F, F, F, F, F, F) {
    let t52613 = t12133 * t498;
    let t52649 = t16848 * t498;
    let t52696 = t613 * t12159;
    let t52697 = t1938 * t1380;
    let t52852 = t1924 * t12230;
    let t52930 = t6220 * t4479;
    let t52933 = t17306 * t1505;
    let t52955 = t18266 * t1628;
    let t53436 = t6183 * t1610;
    (t52613, t52649, t52696, t52697, t52852, t52930, t52933, t52955, t53436)
}
