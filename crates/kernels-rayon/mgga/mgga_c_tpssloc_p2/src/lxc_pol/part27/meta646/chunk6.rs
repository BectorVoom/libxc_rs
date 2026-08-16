//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2225/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2225(t1054: f64, t4693: f64, t13783: f64, t1926: f64, t221: f64, t25432: f64, t10164: f64, t10170: f64, t1052: f64, t1065: f64, t14658: f64, t1955: f64, t23327: f64, t23329: f64, t23330: f64, t23369: f64, t23402: f64, t23581: f64, t25429: f64, t25705: f64, t25749: f64, t25757: f64, t25801: f64, t25810: f64, t2771: f64, t2780: f64, t3174: f64, t388: f64, t3966: f64, t4664: f64, t4694: f64, t6687: f64, t6815: f64, t7554: f64, t7600: f64, t82382: f64, t83285: f64, t83287: f64, t884: f64, t990: f64) -> (f64, f64, f64) {
    let t88804 = t1054 * t4693;
    let t88810 = t1926 * t221 * t13783;
    let t88812 = 0.24369393582936687948e-2_f64 * t88810 * t25432;
    let t88827 = 0.14621636149762012769e-1_f64 * t83285 + 0.14621636149762012769e-1_f64 * t83287 + 2.0_f64 * t10170 * t7600 + 2.0_f64 * t1052 * t3174 * t1955 * t14658 + 2.0_f64 * t990 * t25705 * t388 - 0.54831135561607547884e-2_f64 * t6687 * t25810 * t23402 - 2.0_f64 * t23369 * t4694 - 0.27415567780803773942e-2_f64 * t23327 * t23329 * t25749 * t2780 - 0.36554090374405031923e-2_f64 * t25429 * t23329 * t25749 * t2771 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t88804 * t884 + t88812 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t23330 * t3966 * t1065 - 12.0_f64 * t25757 * t10164 * t6815 * t4664 + 0.26806332941230356743e-1_f64 * t82382 * t7554 + 0.54831135561607547884e-2_f64 * t6687 * t23581 * t25801;
    (t88804, t88810, t88827)
}
