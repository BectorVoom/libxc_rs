//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 511/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk511<F: Float>(t319: F, t652: F, t98: F, t895: F, t902: F, t309: F, t331: F, t330: F, t1849: F, t332: F, t1572: F, t1855: F, t1858: F, t2398: F, t2404: F, t2410: F, t2413: F, t304: F, t308: F, t647: F, t880: F, t885: F, t908: F, t919: F, tau0: F) -> F {
    let t2416 = t319 * t652;
    let t2418 = F::new(1.0) / t98 / t2416;
    let t2419 = t895 * t2418;
    let t2420 = t902 * tau0;
    let t2421 = t2419 * t2420;
    let t2425 = F::new(1.0) / t331 / t309;
    let t2426 = t330 * t2425;
    let t2442 = t332 * t1849;
    let t2446 = -F::new(0.24639784761276436038e1) * t908 * t2398 - F::new(0.24639784761276436038e1) * t919 * t2398 + F::new(0.3553815109799485967e0) * t2404 * t2410 + F::new(0.94768402927986292454e0) * t2413 * t2410 + F::new(0.10670320988213624232e1) * t908 * t2421 + F::new(0.59230251829991432783e0) * t2426 * t2410 + F::new(0.10670320988213624232e1) * t919 * t2421 - F::new(40.0) / F::new(9.0) * t304 * t1858 - F::new(80.0) / F::new(9.0) * t880 * t1858 - F::new(80.0) / F::new(9.0) * t308 * t885 * t647 + F::new(50.0) / F::new(9.0) * t304 * t1855 + F::new(200.0) / F::new(9.0) * t880 * t1855 + F::new(50.0) / F::new(3.0) * t308 * t2442 * t1572;
    t2446
}
