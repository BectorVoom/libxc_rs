//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 735/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk735<F: Float>(t17558: F, t637: F, t643: F, t15625: F, t632: F, t72: F, t4872: F, t8618: F, t4861: F, t8675: F, t358: F, t4883: F, t363: F, t2266: F, t8680: F, t1073: F, t3052: F) -> (F, F, F, F, F, F, F) {
    let t17560 = t637 * t17558 * t643;
    let t17564 = t72 * t632 * t15625;
    let t17567 = t8618 * t4872;
    let t17569 = t637 * t17567 * t643;
    let t17573 = t8675 * t4861;
    let t17575 = t4883 * t358;
    let t17576 = t17575 * t363;
    let t17577 = t2266 * t17576;
    let t17581 = t4872 * t358;
    let t17583 = t8680 * t17581 * t363;
    let t17586 = t2266 * t3052 * t1073;
    (t17560, t17564, t17569, t17573, t17577, t17583, t17586)
}
