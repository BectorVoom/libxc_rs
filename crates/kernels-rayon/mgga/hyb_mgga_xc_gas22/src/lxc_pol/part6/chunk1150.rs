//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1150/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1150(t1171: f64, t4574: f64, t3778: f64, t510: f64, t1543: f64, t2903: f64, t11575: f64, t11578: f64, t11583: f64, t11586: f64, t11589: f64, t11594: f64, t11597: f64, t1163: f64, t3697: f64, t3714: f64, t3786: f64, t4541: f64, t9598: f64, t9612: f64, t9624: f64, t9625: f64, t9737: f64, t9742: f64, t9765: f64, t9769: f64, t9773: f64) -> (f64, f64, f64, f64) {
    let t11605 = t4574 * t1171;
    let t11608 = t510 * t3778;
    let t11611 = t2903 * t1543;
    let t11614 = t3786 * t4541 - 360.0_f64 * t9769 * t11575 + 504.0_f64 * t9773 * t11578 + 24.0_f64 * t9765 * t11578 + 400.0_f64 * t9612 * t11583 - 400.0_f64 * t11586 * t3714 + 1400.0_f64 / 3.0_f64 * t11589 * t3714 + 400.0_f64 / 9.0_f64 * t9598 * t11583 - 400.0_f64 / 9.0_f64 * t11594 * t3714 + 12.0_f64 * t9742 * t11597 - 180.0_f64 * t9624 * t9625 * t3697 + 252.0_f64 * t9737 * t11597 - 8.0_f64 / 9.0_f64 * t11605 * t1163 + 200.0_f64 / 3.0_f64 * t11608 * t3714 - 1000.0_f64 / 3.0_f64 * t11611 * t3714;
    (t11605, t11608, t11611, t11614)
}
