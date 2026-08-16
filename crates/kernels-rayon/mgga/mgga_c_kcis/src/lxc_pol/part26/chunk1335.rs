//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1335/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1335(t5748: f64, t5929: f64, t3738: f64, t7332: f64, t1395: f64, t22422: f64, t22364: f64, t27544: f64, t22649: f64, t97706: f64, t576: f64, t5905: f64, t97800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102875 = t5748 * t5929;
    let t102877 = t3738 * t7332;
    let t102879 = t1395 * t22422;
    let t102881 = t27544 * t22364;
    let t102883 = t97706 * t22649;
    let t102886 = t576 * t97800 * t5905;
    (t102875, t102877, t102879, t102881, t102883, t102886)
}
