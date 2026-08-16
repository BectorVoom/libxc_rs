//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 874/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk874(t17576: f64, t2266: f64, t358: f64, t4872: f64, t363: f64, t8680: f64, t1073: f64, t3052: f64, t4458: f64, t643: f64, t15752: f64, t3621: f64) -> (f64, f64, f64, f64, f64) {
    let t17577 = t2266 * t17576;
    let t17581 = t4872 * t358;
    let t17583 = t8680 * t17581 * t363;
    let t17586 = t2266 * t3052 * t1073;
    let t17590 = t2266 * t4458 * t643;
    let t17593 = t3621 * t15752;
    (t17577, t17583, t17586, t17590, t17593)
}
