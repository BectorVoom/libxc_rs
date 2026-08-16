//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2259/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2259(t13065: f64, t1492: f64, t1527: f64, t1912: f64, t23281: f64, t25160: f64, t25188: f64, t25329: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t28406: f64, t28432: f64, t4301: f64, t5658: f64, t58143: f64, t59466: f64, t59519: f64, t7538: f64, t798: f64, t82147: f64, t82154: f64, t855: f64, t858: f64, t87029: f64, t87050: f64, t87754: f64, t98315: f64, t98319: f64, t98322: f64, t98370: f64, t98409: f64, t98450: f64, t98497: f64, t98536: f64, t98566: f64, t98587: f64, t98886: f64) -> f64 {
    let t98913 = -t2597 * t28432 - 0.16449340668482264365e-1_f64 * t98315 - 0.16449340668482264365e-1_f64 * t98319 + 0.82246703342411321825e-2_f64 * t98322 - t855 * t858 * (t98370 + t98409 + t98450 + t98497 + t98536 + t98566 + t98587 + t98886) - 2.0_f64 * t13065 * t7538 - 2.0_f64 * t59519 * t1912 + t87029 - 0.26044789391763585244e-1_f64 * t82147 + 4.0_f64 * t855 * t2718 * t25329 * t1527 - t23281 * t5658 - t59466 * t1912 - 0.23029076935875170111e0_f64 * t87050 - t82154 - t87754 - t2713 * t28432 + 2.0_f64 * t1492 * t25160 * t259 + t798 * t28406 * t259 - 2.0_f64 * t25188 * t4301 - t58143 * t1912;
    t98913
}
