//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2311/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2311(t100236: f64, t1003: f64, t1022: f64, t17187: f64, t18086: f64, t23346: f64, t23633: f64, t23635: f64, t25500: f64, t28634: f64, t28653: f64, t28660: f64, t353: f64, t383: f64, t4542: f64, t4669: f64, t5398: f64, t6687: f64, t6784: f64, t6785: f64, t6800: f64, t6811: f64, t7614: f64, t82668: f64, t83233: f64, t89329: f64, t99859: f64) -> f64 {
    let t100314 = 0.27415567780803773942e-2_f64 * t23633 * t23635 * t5398 * t1022 * t6800 + t89329 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t7614 + 2.0_f64 * t4669 * t25500 + t1003 * t28634 - 0.54831135561607547884e-2_f64 * t23633 * t83233 * t100236 + t353 * t383 * t99859 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t6785 * t17187 - 0.14621636149762012769e-1_f64 * t82668 * t28653 + 0.21932454224643019153e-1_f64 * t23346 * t28660 + t18086 * t6811;
    t100314
}
