//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2240/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240(t7604: f64, t82632: f64, t25723: f64, t88810: f64, t1409: f64, t3040: f64, t1539: f64, t6746: f64, t82655: f64, t14220: f64, t7581: f64, t11034: f64, t1599: f64, t1629: f64, t23346: f64, t23518: f64, t23604: f64, t23620: f64, t23633: f64, t25467: f64, t25567: f64, t25659: f64, t25708: f64, t3186: f64, t4673: f64, t6687: f64, t82382: f64, t82653: f64, t82789: f64, t83233: f64, t83245: f64, t83265: f64, t89106: f64) -> (f64, f64) {
    let t89366 = t82632 * t7604;
    let t89369 = 0.24369393582936687948e-2_f64 * t88810 * t25723;
    let t89375 = t1409 * t3040;
    let t89395 = t82655 * t1539 * t6746;
    let t89399 = t82655 * t7581 * t14220;
    let t89402 = 0.26806332941230356743e-1_f64 * t82382 * t7604 - 0.60923483957341719871e-3_f64 * t89366 + t89369 + 4.0_f64 * t11034 * t25708 - 0.82246703342411321825e-2_f64 * t6687 * t1599 * t23620 - 0.27415567780803773942e-2_f64 * t83245 * t83265 * t89375 * t23604 + 4.0_f64 * t3186 * t25567 * t4673 + 0.43864908449286038306e-1_f64 * t23346 * t25467 - 0.27415567780803773942e-2_f64 * t82789 - 0.54831135561607547884e-2_f64 * t83245 * t23518 * t1629 * t25659 * t14220 - 0.10966227112321509577e-1_f64 * t23633 * t83233 * t89106 - 0.54831135561607547884e-2_f64 * t82653 * t89395 - 0.54831135561607547884e-2_f64 * t82653 * t89399;
    (t89375, t89402)
}
