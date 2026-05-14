//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1144/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1144<F: Float>(t1085: F, t1099: F, t23583: F, t7591: F, t2683: F, t2689: F, t2693: F, t7531: F, t2697: F, t7755: F, t1044: F, t25: F, t12: F, t222: F, t22513: F, t438: F) -> (F, F, F, F, F, F, F) {
    let t23587 = 0.14035736694323150897e2 * t1099 * t7591 * t23583 * t1085;
    let t23588 = t2683 * t2683;
    let t23592 = 0.51947577317044391277e2 * t1099 * t2689 * t23588 * t2693;
    let t23596 = 0.6233709278045326953e3 * t1099 * t7531 * t23583 * t2693;
    let t23597 = t2697 * t7755;
    let t23604 = 1.0 / t25 / t1044;
    let t23606 = 1.0 / t438 / t22513 * t12 * t23604 * t222 / 48.0;
    (t23587, t23588, t23592, t23596, t23597, t23604, t23606)
}
