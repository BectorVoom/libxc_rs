//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1447/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1447(t1117: f64, t3778: f64, t1849: f64, t531: f64, t1620: f64, t31155: f64, t1160: f64, t4576: f64, t9656: f64, t1134: f64, t14765: f64, t14770: f64, t14775: f64, t14815: f64, t14818: f64, t1543: f64, t22754: f64, t30733: f64, t31483: f64, t3767: f64, t510: f64, t7817: f64, t9523: f64, t9535: f64, t9639: f64, t9750: f64) -> (f64, f64, f64) {
    let t31512 = t1117 * t3778;
    let t31526 = t531 * t1849;
    let t31528 = t31526 * t31155 * t1620;
    let t31539 = t4576 * t1160;
    let t31540 = t31539 * t9656;
    let t31545 = 800.0_f64 / 3.0_f64 * t31512 * t9523 - 1600.0_f64 / 3.0_f64 * t510 * t9750 * t9535 - 8000.0_f64 / 3.0_f64 * t7817 * t1543 * t9523 + 4000.0_f64 * t30733 * t9535 + 5600.0_f64 * t1134 * t3767 * t9523 + 20000.0_f64 / 81.0_f64 * t14775 * t31528 + 5000.0_f64 / 27.0_f64 * t14815 * t31528 + 25000.0_f64 / 27.0_f64 * t14818 * t31528 + 20000.0_f64 / 9.0_f64 * t14765 * t31528 + 35000.0_f64 / 27.0_f64 * t14770 * t31528 - 64.0_f64 / 3.0_f64 * t22754 * t31540 + 256.0_f64 / 9.0_f64 * t9639 * t31483;
    (t31539, t31540, t31545)
}
