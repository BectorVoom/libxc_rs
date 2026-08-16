//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2227/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2227(t4657: f64, t6688: f64, t7566: f64, t82632: f64, t23384: f64, t25400: f64, t13611: f64, t13933: f64, t13939: f64, t14552: f64, t1922: f64, t1945: f64, t23323: f64, t23346: f64, t23372: f64, t23725: f64, t25420: f64, t25755: f64, t25827: f64, t3026: f64, t3176: f64, t388: f64, t4557: f64, t4694: f64, t6687: f64, t6689: f64, t6690: f64, t6691: f64, t6776: f64, t7562: f64, t83329: f64) -> f64 {
    let t88868 = t6688 * t4657;
    let t88882 = t82632 * t7566;
    let t88889 = 0.54831135561607547884e-2_f64 * t23384 * t25400;
    let t88900 = 0.54831135561607547884e-2_f64 * t6687 * t88868 * t6691 + 4.0_f64 * t4557 * t23725 + 4.0_f64 * t3026 * t25420 + 0.27415567780803773942e-2_f64 * t6687 * t6689 * t6690 * t13611 + 0.43864908449286038306e-1_f64 * t23346 * t25827 + 0.18277045187202515961e-2_f64 * t88882 + 4.0_f64 * t14552 * t6776 + 2.0_f64 * t25755 * t3176 - t88889 + 0.80418998823691070228e-1_f64 * t23323 * t7562 - 0.18277045187202515961e-2_f64 * t83329 - 2.0_f64 * t23372 * t4694 + t13939 * t1945 * t388 - 0.82246703342411321825e-2_f64 * t6687 * t13933 * t1922;
    t88900
}
