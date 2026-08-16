//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 511/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk511(t319: f64, t652: f64, t98: f64, t895: f64, t902: f64, t309: f64, t331: f64, t330: f64, t1849: f64, t332: f64, t1572: f64, t1855: f64, t1858: f64, t2398: f64, t2404: f64, t2410: f64, t2413: f64, t304: f64, t308: f64, t647: f64, t880: f64, t885: f64, t908: f64, t919: f64, tau0: f64) -> f64 {
    let t2416 = t319 * t652;
    let t2418 = 1.0_f64 / t98 / t2416;
    let t2419 = t895 * t2418;
    let t2420 = t902 * tau0;
    let t2421 = t2419 * t2420;
    let t2425 = 1.0_f64 / t331 / t309;
    let t2426 = t330 * t2425;
    let t2442 = t332 * t1849;
    let t2446 = -0.24639784761276436038e1_f64 * t908 * t2398 - 0.24639784761276436038e1_f64 * t919 * t2398 + 0.3553815109799485967e0_f64 * t2404 * t2410 + 0.94768402927986292454e0_f64 * t2413 * t2410 + 0.10670320988213624232e1_f64 * t908 * t2421 + 0.59230251829991432783e0_f64 * t2426 * t2410 + 0.10670320988213624232e1_f64 * t919 * t2421 - 40.0_f64 / 9.0_f64 * t304 * t1858 - 80.0_f64 / 9.0_f64 * t880 * t1858 - 80.0_f64 / 9.0_f64 * t308 * t885 * t647 + 50.0_f64 / 9.0_f64 * t304 * t1855 + 200.0_f64 / 9.0_f64 * t880 * t1855 + 50.0_f64 / 3.0_f64 * t308 * t2442 * t1572;
    t2446
}
