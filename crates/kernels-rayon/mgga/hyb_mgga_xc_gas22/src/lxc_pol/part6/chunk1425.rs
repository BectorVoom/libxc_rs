//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1425/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1425(t2849: f64, t4540: f64, t9691: f64, t11524: f64, t2824: f64, t30771: f64, t9501: f64, t11315: f64, t11475: f64, t11536: f64, t26194: f64, t26421: f64, t26429: f64, t2834: f64, t2910: f64, t30787: f64, t30790: f64, t30818: f64, t30822: f64, t3680: f64, t3688: f64, t3757: f64, t4565: f64, t4571: f64, t7800: f64, t7811: f64, t9575: f64, t9696: f64, t9747: f64) -> (f64, f64, f64, f64) {
    let t30841 = t2849 * t4540 * t9691;
    let t30854 = t11524 * t2824;
    let t30860 = t9501 * t30771;
    let t30867 = -160.0_f64 / 9.0_f64 * t7800 * t11315 * t9696 - 64.0_f64 / 27.0_f64 * t3680 * t30841 - 32.0_f64 / 9.0_f64 * t2834 * t30818 + 64.0_f64 / 27.0_f64 * t3688 * t30822 - 4.0_f64 * t4571 * t2910 + 2.0_f64 * t9747 * t4565 - 5600.0_f64 / 9.0_f64 * t9575 * t30790 + 64.0_f64 / 3.0_f64 * t26429 * t30854 - 320.0_f64 / 3.0_f64 * t26421 * t11536 * t2824 + 704.0_f64 / 81.0_f64 * t3757 * t30860 + 64.0_f64 / 9.0_f64 * t26194 * t11475 + 32.0_f64 / 9.0_f64 * t7811 * t30787;
    (t30841, t30854, t30860, t30867)
}
