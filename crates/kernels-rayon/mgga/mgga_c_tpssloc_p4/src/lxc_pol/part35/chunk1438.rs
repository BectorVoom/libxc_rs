//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1438/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1438(t103507: f64, t103520: f64, t103546: f64, t103573: f64, t103610: f64, t21510: f64, t22340: f64, t22348: f64, t22364: f64, t22386: f64, t24589: f64, t24812: f64, t24814: f64, t24815: f64, t27516: f64, t27536: f64, t27549: f64, t27550: f64, t27551: f64, t29740: f64, t29744: f64, t29762: f64, t7373: f64, t7375: f64, t7376: f64, t8066: f64, t85963: f64, t85965: f64, t85966: f64, t94784: f64) -> f64 {
    let t109206 = 0.16449340668482264365e-1_f64 * t24589 * t27516 * t29740 + 0.16449340668482264365e-1_f64 * t103507 - 0.54831135561607547884e-2_f64 * t94784 + 0.10966227112321509577e-1_f64 * t27549 * t27550 * t27551 * t21510 + 0.82246703342411321826e-2_f64 * t24589 * t103520 * t8066 + 0.16449340668482264365e-1_f64 * t103546 - 0.54831135561607547883e-2_f64 * t103573 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t22386 * t7376 - 0.82246703342411321826e-2_f64 * t103610 + 0.24674011002723396548e-1_f64 * t7373 * t7375 * t22340 * t7376 + 0.49348022005446793095e-1_f64 * t24812 * t24814 * t22364 * t24815 + 0.49348022005446793095e-1_f64 * t85963 * t85965 * t22348 * t85966 - 0.24674011002723396548e-1_f64 * t7373 * t27536 * t29744 + 0.16449340668482264365e-1_f64 * t24589 * t27516 * t29762;
    t109206
}
