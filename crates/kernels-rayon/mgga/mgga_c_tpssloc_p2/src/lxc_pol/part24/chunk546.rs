//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 546/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk546(t919: f64, t923: f64, t307: f64, t922: f64, t302: f64, t931: f64, t932: f64, t2764: f64, t2822: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2816: f64, t2818: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    let t2860 = 1.0_f64 / t2859;
    let t2861 = t302 * t2860;
    let t2862 = t931 * t931;
    let t2863 = t2862 * t932;
    let t2868 = 0.68863333333333333333e0_f64 * t2764;
    let t2875 = 0.17365833333333333333e0_f64 * t2822;
    let t2880 = -0.17648625e1_f64 * t2800 + 0.3529725e1_f64 * t2808 + t2868 + 0.34431666666666666666e0_f64 * t2766 - 0.34431666666666666667e0_f64 * t2773 + 0.103295e1_f64 * t2778 - 0.516475e0_f64 * t2782 + 0.31558125e0_f64 * t2816 + 0.6311625e0_f64 * t2818 + t2875 + 0.13892666666666666667e0_f64 * t2824 - 0.34731666666666666667e-1_f64 * t2828 + 0.20839e0_f64 * t2831 - 0.104195e0_f64 * t2834;
    (t2856, t2860, t2861, t2862, t2863, t2880)
}
