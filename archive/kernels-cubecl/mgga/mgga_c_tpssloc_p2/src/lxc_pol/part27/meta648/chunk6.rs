//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2244/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2244<F: Float>(t23478: F, t4547: F, t7607: F, t82573: F, t1058: F, t1060: F, t11051: F, t11054: F, t14608: F, t23327: F, t23346: F, t23633: F, t23654: F, t23662: F, t25493: F, t25518: F, t25549: F, t3016: F, t3186: F, t353: F, t383: F, t4649: F, t4669: F, t6687: F, t6768: F, t6786: F, t7614: F, t7619: F, t7620: F, t82382: F, t82534: F, t82625: F, t88728: F) -> F {
    let t89532 = t4547 * t23478;
    let t89546 = F::cast_from(0.14621636149762012769e-1_f64) * t82573 * t7607;
    let t89547 = F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t82625 * t25549 + F::cast_from(2.0_f64) * t1058 * t6768 * t4649 * t1060 - t14608 * t23662 + t11051 * t7620 + F::cast_from(2.0_f64) * t4669 * t23654 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3016 * t7614 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t25518 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t89532 * t6786 + F::cast_from(0.43864908449286038306e-1_f64) * t82534 * t25493 + t353 * t383 * t88728 + F::cast_from(2.0_f64) * t3186 * t7619 * t11054 - F::cast_from(0.80418998823691070228e-1_f64) * t82382 * t7607 + t89546;
    t89547
}
