//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1424/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1424(t11282: f64, t9696: f64, t2828: f64, t4574: f64, t11319: f64, t2849: f64, t4544: f64, t9691: f64, t4524: f64, t11266: f64, t11320: f64, t14815: f64, t14818: f64, t2821: f64, t2831: f64, t2838: f64, t2900: f64, t2940: f64, t30790: f64, t3661: f64, t4530: f64, t4568: f64, t4571: f64, t518: f64, t7643: f64, t9436: f64, t9521: f64, t9535: f64, t9700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30807 = t11282 * t9696;
    let t30813 = t4574 * t2828;
    let t30818 = t11319 * t9696;
    let t30821 = t2849 * t4544;
    let t30822 = t30821 * t9691;
    let t30825 = t2849 * t4524;
    let t30826 = t30825 * t9691;
    let t30829 = t11266 * t9696;
    let t30836 = -1600.0_f64 / 27.0_f64 * t9521 * t30790 + 12.0_f64 * t4571 * t2900 - 4.0_f64 * t2940 * t4568 + 32.0_f64 / 9.0_f64 * t2838 * t30807 - 22400.0_f64 / 3.0_f64 * t518 * t9436 * t9535 + 32.0_f64 / 9.0_f64 * t30813 * t2831 - 16.0_f64 / 9.0_f64 * t9700 * t11320 - 32.0_f64 / 27.0_f64 * t2821 * t30818 + 64.0_f64 / 81.0_f64 * t3661 * t30822 - 64.0_f64 / 27.0_f64 * t14815 * t30826 - 32.0_f64 / 9.0_f64 * t7643 * t30829 - 320.0_f64 / 27.0_f64 * t14818 * t2849 * t4530 * t9691;
    (t30807, t30818, t30822, t30826, t30829, t30836)
}
