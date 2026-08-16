//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1362/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1362(t25270: f64, t8986: f64, t8993: f64, t9104: f64, t25262: f64, t8997: f64, t2515: f64, t2521: f64, t4244: f64, t2479: f64, t4273: f64, t7148: f64) -> (f64, f64, f64, f64, f64) {
    let t29656 = 0.19298375398431042081e3_f64 * t25270 * t8986;
    let t29658 = 0.32163958997385070134e2_f64 * t9104 * t8993;
    let t29660 = 0.1034520258385468006e4_f64 * t25262 * t8997;
    let t29663 = 6.0_f64 * t2521 * t4244 * t2515;
    let t29666 = 0.57895126195293126241e3_f64 * t7148 * t4273 * t2479;
    (t29656, t29658, t29660, t29663, t29666)
}
