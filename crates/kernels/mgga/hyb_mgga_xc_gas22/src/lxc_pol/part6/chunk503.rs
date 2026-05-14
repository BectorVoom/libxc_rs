//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 503/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk503<F: Float>(t2419: F, t2420: F, t309: F, t331: F, t330: F, t1849: F, t332: F, t1572: F, t1855: F, t1858: F, t2398: F, t2404: F, t2410: F, t2413: F, t304: F, t308: F, t647: F, t880: F, t885: F, t908: F, t919: F) -> (F,) {
    let t2421 = t2419 * t2420;
    let t2425 = 1.0 / t331 / t309;
    let t2426 = t330 * t2425;
    let t2442 = t332 * t1849;
    let t2446 = -0.24639784761276436038e1 * t908 * t2398 - 0.24639784761276436038e1 * t919 * t2398 + 0.3553815109799485967e0 * t2404 * t2410 + 0.94768402927986292454e0 * t2413 * t2410 + 0.10670320988213624232e1 * t908 * t2421 + 0.59230251829991432783e0 * t2426 * t2410 + 0.10670320988213624232e1 * t919 * t2421 - 40.0 / 9.0 * t304 * t1858 - 80.0 / 9.0 * t880 * t1858 - 80.0 / 9.0 * t308 * t885 * t647 + 50.0 / 9.0 * t304 * t1855 + 200.0 / 9.0 * t880 * t1855 + 50.0 / 3.0 * t308 * t2442 * t1572;
    (t2446,)
}
