//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 936/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk936(t118: f64, t181: f64, t2454: f64, t2459: f64, t2460: f64, t2462: f64, t2471: f64, t2472: f64, t2477: f64, t2479: f64, t2480: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2510: f64, t2513: f64, t268: f64, t676: f64, t730: f64, t732: f64, t747: f64, t9697: f64, t9799: f64, t9803: f64, t9810: f64, t9814: f64, t9820: f64, t9824: f64, t9828: f64, t9844: f64, t9847: f64, t9853: f64, t9859: f64) -> f64 {
    let t9860 = 0.32530743900905219526e-1_f64 * t268 * t9799 * t2495 + 0.21687162600603479684e-1_f64 * t268 * t9803 * t747 - 0.16265371950452609763e-1_f64 * t268 * t2490 * t2505 - 0.48159733137676571078e0_f64 * t268 * t9810 * t2513 + 0.68493333333333333332e-1_f64 * t268 * t9814 * t732 + t9820 + t9824 - 0.51369999999999999999e-1_f64 * t268 * t2454 * t2472 - 0.16522625736956710527e1_f64 * t268 * t9828 * t2480 + 0.10274e0_f64 * t268 * t676 * t2459 * t2462 + 0.96491876992155210402e2_f64 * t2477 * t2471 * t2479 * t730 - 6.0_f64 * t2460 * t732 * t2471 + 0.51947577317044391277e2_f64 * t2510 * t9844 - 0.35089341735807877242e1_f64 * t2494 * t9847 - t9853 + 0.56968947174242584612e-3_f64 * t118 * t9697 * t181 - t9859;
    t9860
}
