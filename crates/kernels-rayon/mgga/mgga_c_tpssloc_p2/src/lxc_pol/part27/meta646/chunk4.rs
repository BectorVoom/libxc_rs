//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2223/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2223(t7554: f64, t82632: f64, t14529: f64, t14545: f64, t23327: f64, t23341: f64, t23346: f64, t23395: f64, t25406: f64, t25413: f64, t25732: f64, t25784: f64, t3016: f64, t3026: f64, t349: f64, t388: f64, t4660: f64, t6687: f64, t6816: f64, t7553: f64, t7565: f64, t82437: f64, t82463: f64, t82490: f64, t83296: f64, t83303: f64, t88728: f64) -> f64 {
    let t88731 = t82632 * t7554;
    let t88742 = -0.14621636149762012769e-1_f64 * t82437 - 2.0_f64 * t3026 * t25732 + 0.82246703342411321825e-2_f64 * t6687 * t3016 * t25784 + 0.43864908449286038306e-1_f64 * t23346 * t25413 + 0.16449340668482264365e-1_f64 * t6687 * t25406 * t23395 - 2.0_f64 * t14545 * t6816 + 0.27415567780803773942e-2_f64 * t82463 - 6.0_f64 * t4660 * t23341 + t349 * t88728 * t388 - 0.60923483957341719871e-3_f64 * t88731 - 2.0_f64 * t14529 * t6816 - 0.82246703342411321825e-2_f64 * t6687 * t83296 * t7565 - 0.54831135561607547884e-2_f64 * t23327 * t83303 * t7553 + 0.12184696791468343974e-2_f64 * t82490;
    t88742
}
