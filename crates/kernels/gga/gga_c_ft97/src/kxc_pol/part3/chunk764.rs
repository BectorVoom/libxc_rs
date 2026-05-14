//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 764/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk764<F: Float>(t17576: F, t2266: F, t358: F, t4872: F, t363: F, t8680: F, t1073: F, t3052: F, t4458: F, t643: F, t15752: F, t3621: F, t15756: F, t4462: F, t15768: F, t15763: F, t3613: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17577 = t2266 * t17576;
    let t17581 = t4872 * t358;
    let t17583 = t8680 * t17581 * t363;
    let t17586 = t2266 * t3052 * t1073;
    let t17590 = t2266 * t4458 * t643;
    let t17593 = t3621 * t15752;
    let t17595 = t3621 * t15756;
    let t17599 = t2266 * t4462 * t643;
    let t17602 = t3621 * t15768;
    let t17605 = t3613 * t15763;
    (t17577, t17583, t17586, t17590, t17593, t17595, t17599, t17602, t17605)
}
