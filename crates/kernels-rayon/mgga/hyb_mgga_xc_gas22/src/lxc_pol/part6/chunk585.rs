//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 585/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk585(t2757: f64, t496: f64, t221: f64, t2662: f64, t454: f64, t1074: f64, t567: f64, t1073: f64, t475: f64, t470: f64, t1080: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2759 = 32.0_f64 * t2757 * t496;
    let t2762 = 0.14764627977777777777e-2_f64 * t221 * t2662 * t454;
    let t2766 = t567 * t1074;
    let t2770 = t1073 * t475;
    let t2771 = 1.0_f64 / t2770;
    let t2772 = t470 * t2771;
    let t2773 = t1080 * t1080;
    (t2759, t2762, t2766, t2770, t2771, t2772, t2773)
}
