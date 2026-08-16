//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2331/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2331(t477: f64, t5052: f64, t27654: f64, t7327: f64, t24745: f64, t4935: f64, t1090: f64, t1186: f64, t1201: f64, t1215: f64, t15771: f64, t2121: f64, t2147: f64, t24589: f64, t24799: f64, t24849: f64, t24851: f64, t27406: f64, t27525: f64, t27549: f64, t27552: f64, t27722: f64, t27732: f64, t3966: f64, t462: f64, t7283: f64, t7362: f64, t7364: f64, t7373: f64, t7376: f64, t7377: f64, t86106: f64, t86113: f64, t86116: f64, t94976: f64) -> f64 {
    let t95794 = t477 * t5052;
    let t95803 = t27654 * t7327;
    let t95813 = t4935 * t24745;
    let t95817 = 0.82246703342411321825e-2_f64 * t2121 * t462 * t2147 * t15771 - 0.54831135561607547884e-2_f64 * t24849 * t24851 * t3966 * t1215 * t7376 + 2.0_f64 * t1201 * t27722 + 0.14621636149762012769e-1_f64 * t27406 * t24799 + 0.12184696791468343974e-2_f64 * t86106 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t95794 * t1090 - 0.27415567780803773942e-2_f64 * t86113 - 0.73108180748810063846e-2_f64 * t27549 * t94976 * t27552 - 0.16449340668482264365e-1_f64 * t7373 * t95803 * t7377 - 0.16449340668482264365e-1_f64 * t7283 * t1186 * t27732 - 0.54831135561607547884e-2_f64 * t24849 * t86116 * t27525 + 0.54831135561607547884e-2_f64 * t24589 * t95813 * t7364;
    t95817
}
