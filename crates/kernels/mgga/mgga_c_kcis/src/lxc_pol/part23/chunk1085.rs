//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1085/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1085<F: Float>(t3738: F, t5935: F, t1528: F, t1928: F, t7953: F, t17457: F, t27544: F, t1468: F, t17703: F, t17467: F, t4254: F, t572: F, t15956: F, t17383: F, t7952: F, t27517: F, t5910: F) -> (F, F, F, F, F, F, F, F) {
    let t97782 = t3738 * t5935;
    let t97784 = t1528 * t1928;
    let t97785 = t97784 * t7953;
    let t97787 = t27544 * t17457;
    let t97789 = t1468 * t17703;
    let t97791 = t27544 * t17467;
    let t97793 = t4254 * t572;
    let t97794 = t97793 * t15956;
    let t97796 = t7952 * t17383;
    let t97798 = t27517 * t5910;
    (t97782, t97785, t97787, t97789, t97791, t97794, t97796, t97798)
}
