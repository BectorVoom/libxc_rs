//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 983/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk983(t6562: f64, t82133: f64, t8547: f64, t7106: f64, t857: f64, t22986: f64, t23270: f64, t776: f64, t112723: f64, t112727: f64, t112730: f64, t112733: f64, t112742: f64, t112744: f64, t114772: f64, t114781: f64, t114785: f64, t114792: f64, t23214: f64, t25168: f64, t2713: f64, t2718: f64, t31409: f64, t31416: f64, t6662: f64, t855: f64, t8553: f64, t866: f64, t87013: f64, t92394: f64, t9593: f64) -> f64 {
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114800 = t22986 * t23270 * t114797 * t776;
    let t114802 = t112723 + 24.0_f64 * t25168 * t92394 * t23214 + 0.3289868133696452873e-1_f64 * t114772 - 12.0_f64 * t87013 * t31416 + 4.0_f64 * t855 * t2718 * t7106 * t6662 + t112727 - t112730 + t112733 - 0.82246703342411321825e-2_f64 * t114781 + 4.0_f64 * t9593 * t8553 - 2.0_f64 * t114785 * t866 + 4.0_f64 * t2713 * t31409 + t112742 + t112744 + 0.82246703342411321824e-2_f64 * t114792 + 0.82246703342411321824e-2_f64 * t114795 + 0.3289868133696452873e-1_f64 * t114800;
    t114802
}
