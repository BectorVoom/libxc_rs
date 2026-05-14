//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1293/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1293<F: Float>(t22709: F, t6139: F, t7340: F, t19820: F, t2122: F, t22749: F, t24165: F, t24196: F, t24224: F, t24288: F, t24292: F, t24298: F, t24300: F, t24305: F, t24318: F, t24323: F, t24326: F, t2551: F, t2573: F, t5108: F, t5109: F, t6583: F, t7321: F, t7334: F, t7337: F) -> (F,) {
    let t24329 = t6139 * t22709 * t7340;
    let t24331 = 0.16463622957338778996e0 * t2122 * t7321 * t24224 + 0.16463622957338778996e0 * t2122 * t7321 * t24288 + 0.19756347548806534797e1 * t22749 * t7321 * t24292 - 0.7801399566048841707e0 * t19820 * t7334 + 0.23049072140274290595e1 * t24298 + 0.16463622957338778996e0 * t2122 * t7321 * t24300 + 0.13869154784086829701e1 * t24305 - 0.26004665220162805689e0 * t6583 * t5109 * t24165 * t2573 - 0.39006997830244208535e0 * t5108 * t5109 * t24165 * t2551 - 0.32927245914677557992e0 * t2122 * t7337 * t24196 - 0.32927245914677557992e0 * t2122 * t7337 * t24318 + 0.20803732176130244552e1 * t24323 + 0.13869154784086829701e1 * t24326 + 0.41607464352260489103e1 * t24329;
    (t24331,)
}
