//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2285/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2285(t27381: f64, t7294: f64, t1715: f64, t3475: f64, t1186: f64, t11928: f64, t1238: f64, t15802: f64, t1760: f64, t2155: f64, t24589: f64, t24597: f64, t24603: f64, t24615: f64, t24616: f64, t24867: f64, t24897: f64, t27406: f64, t27437: f64, t27549: f64, t27751: f64, t27761: f64, t27775: f64, t27799: f64, t3477: f64, t3593: f64, t3598: f64, t4723: f64, t4945: f64, t52386: f64, t7283: f64, t7300: f64, t8010: f64, t8088: f64, t86403: f64, t86415: f64, t94369: f64) -> (f64, f64) {
    let t94584 = t7294 * t27381;
    let t94588 = t1715 * t3475;
    let t94605 = 0.54831135561607547884e-2_f64 * t24589 * t86415 * t27437 - 0.73108180748810063846e-2_f64 * t27549 * t94369 * t4723 * t24603 - 0.73108180748810063846e-2_f64 * t27549 * t86403 * t27775 + 0.16449340668482264365e-1_f64 * t7283 * t7300 * t24615 * t15802 - t11928 * t8088 - 0.9747757433174675179e-2_f64 * t27406 * t24597 + 4.0_f64 * t3593 * t27761 + 0.16449340668482264365e-1_f64 * t7283 * t1186 * t94584 - 0.82246703342411321825e-2_f64 * t7283 * t94588 * t27799 - 6.0_f64 * t4945 * t24897 + 0.16449340668482264365e-1_f64 * t7283 * t27751 * t24616 - t52386 * t2155 - 0.82246703342411321825e-2_f64 * t7283 * t3477 * t8010 + 2.0_f64 * t1238 * t3598 * t24867 * t1760;
    (t94588, t94605)
}
