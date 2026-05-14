//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1291/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1291<F: Float>(t9449: F, t9585: F, t27067: F, t9588: F, t1003: F, t11346: F, t11551: F, t11559: F, t2590: F, t2598: F, t2605: F, t27450: F, t31631: F, t31633: F, t31636: F, t31640: F, t31642: F, t31644: F, t3614: F, t3621: F, t9318: F) -> (F, F, F) {
    let t31646 = 4.0 * t9449 * t9585;
    let t31648 = 0.19298375398431042081e3 * t27067 * t9588;
    let t31649 = -0.70178683471615754484e1 * t2605 * t11551 - 0.34631718211362927518e2 * t1003 * t3621 * t27450 + 0.46785788981077169656e1 * t9318 * t3614 + 0.11696447245269292414e1 * t1003 * t11559 * t2590 - 0.6233709278045326953e3 * t1003 * t11346 * t2598 + t31631 + t31633 + t31636 + t31640 - t31642 - t31644 - t31646 - t31648;
    (t31646, t31648, t31649)
}
