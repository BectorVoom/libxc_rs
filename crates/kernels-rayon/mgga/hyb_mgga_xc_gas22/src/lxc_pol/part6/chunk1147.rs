//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1147/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1147(t513: f64, t5198: f64, t1118: f64, t4535: f64, t1123: f64, t4524: f64, t1145: f64, t1129: f64, t11289: f64, t11293: f64, t1130: f64, t11411: f64, t11479: f64, t11482: f64, t11485: f64, t2834: f64, t2838: f64, t3713: f64, t3714: f64, t3717: f64, t3753: f64, t3788: f64, t4565: f64, t4571: f64, t4577: f64, t7734: f64, t7780: f64, t7806: f64, t9558: f64, t9782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11512 = t5198 * t513;
    let t11515 = t1118 * t4535;
    let t11520 = t4524 * t1123;
    let t11521 = t1145 * t11520;
    let t11524 = t4524 * t1129;
    let t11525 = t1145 * t11524;
    let t11530 = 32.0_f64 * t7806 * t11479 - 112.0_f64 / 3.0_f64 * t9558 * t11482 + 352.0_f64 / 243.0_f64 * t3753 * t11485 - 128.0_f64 / 81.0_f64 * t3753 * t11411 - 4.0_f64 * t4571 * t1130 + 2.0_f64 * t3788 * t4565 + 88.0_f64 / 9.0_f64 * t2834 * t11289 - 88.0_f64 / 9.0_f64 * t2838 * t11293 - 200.0_f64 / 9.0_f64 * t11512 * t3714 + 800.0_f64 / 27.0_f64 * t3713 * t11515 + 800.0_f64 / 27.0_f64 * t3717 * t11515 + 126.0_f64 * t7734 * t11521 - 168.0_f64 * t7780 * t11525 - 4.0_f64 * t9782 * t4577;
    (t11512, t11515, t11520, t11521, t11524, t11525, t11530)
}
