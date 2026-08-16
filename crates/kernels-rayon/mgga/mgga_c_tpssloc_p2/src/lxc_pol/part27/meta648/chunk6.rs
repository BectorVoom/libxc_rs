//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2244/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244(t23478: f64, t4547: f64, t7607: f64, t82573: f64, t1058: f64, t1060: f64, t11051: f64, t11054: f64, t14608: f64, t23327: f64, t23346: f64, t23633: f64, t23654: f64, t23662: f64, t25493: f64, t25518: f64, t25549: f64, t3016: f64, t3186: f64, t353: f64, t383: f64, t4649: f64, t4669: f64, t6687: f64, t6768: f64, t6786: f64, t7614: f64, t7619: f64, t7620: f64, t82382: f64, t82534: f64, t82625: f64, t88728: f64) -> f64 {
    let t89532 = t4547 * t23478;
    let t89546 = 0.14621636149762012769e-1_f64 * t82573 * t7607;
    let t89547 = 0.54831135561607547884e-2_f64 * t23633 * t82625 * t25549 + 2.0_f64 * t1058 * t6768 * t4649 * t1060 - t14608 * t23662 + t11051 * t7620 + 2.0_f64 * t4669 * t23654 - 0.82246703342411321825e-2_f64 * t6687 * t3016 * t7614 - 0.14621636149762012769e-1_f64 * t23346 * t25518 - 0.54831135561607547884e-2_f64 * t23327 * t89532 * t6786 + 0.43864908449286038306e-1_f64 * t82534 * t25493 + t353 * t383 * t88728 + 2.0_f64 * t3186 * t7619 * t11054 - 0.80418998823691070228e-1_f64 * t82382 * t7607 + t89546;
    t89547
}
