//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1423/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1423(t11478: f64, t9685: f64, t11474: f64, t11382: f64, t9489: f64, t11391: f64, t4576: f64, t7785: f64, t22754: f64, t26429: f64, t2829: f64, t30752: f64, t30760: f64, t30764: f64, t30767: f64, t30772: f64, t30777: f64, t30781: f64, t3753: f64, t3757: f64, t7806: f64, t7811: f64, t9533: f64, t9542: f64, t9558: f64, t9587: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30784 = t11478 * t9685;
    let t30787 = t11474 * t9685;
    let t30790 = t11382 * t9489;
    let t30793 = t11391 * t9489;
    let t30796 = t4576 * t7785;
    let t30799 = -88.0_f64 / 27.0_f64 * t2829 * t30752 + 64.0_f64 / 3.0_f64 * t26429 * t30760 - 1936.0_f64 / 243.0_f64 * t3757 * t30764 - 32.0_f64 / 3.0_f64 * t22754 * t30767 + 5632.0_f64 / 2187.0_f64 * t9587 * t30772 - 4096.0_f64 / 729.0_f64 * t9587 * t30777 - 2560.0_f64 / 243.0_f64 * t3753 * t30781 - 112.0_f64 / 3.0_f64 * t9558 * t30784 + 32.0_f64 * t7806 * t30787 - 800.0_f64 / 9.0_f64 * t9533 * t30790 + 4000.0_f64 / 9.0_f64 * t9542 * t30793 - 352.0_f64 / 27.0_f64 * t7811 * t30796;
    (t30784, t30787, t30790, t30793, t30796, t30799)
}
