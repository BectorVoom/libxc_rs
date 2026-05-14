//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 986/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk986<F: Float>(t173: F, t1764: F, t1730: F, t614: F, t1732: F, t6895: F, t167: F, t168: F, t16942: F, t180: F, t66: F, t16405: F, t618: F, t187: F, t5417: F, t1675: F) -> (F, F, F, F, F, F, F, F) {
    let t17051 = t1764 * t173;
    let t17053 = t1730 * t17051 * t614;
    let t17067 = t6895 * t1732;
    let t17088 = 0.28974367305964659283e0 * t167 * t168 / t66 / t16942 * t180;
    let t17095 = t167 * t16405;
    let t17096 = t17095 * t618;
    let t17121 = 1.0 / t5417 / t187;
    let t17244 = t1675 * t1675;
    (t17051, t17053, t17067, t17088, t17095, t17096, t17121, t17244)
}
