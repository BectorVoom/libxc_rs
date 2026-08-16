//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 793/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk793(t11613: f64, t1238: f64, t2121: f64, t2155: f64, t24564: f64, t24568: f64, t24571: f64, t24575: f64, t24577: f64, t24582: f64, t24587: f64, t24589: f64, t24591: f64, t24597: f64, t24605: f64, t24612: f64, t24617: f64, t24626: f64, t3487: f64, t3593: f64, t3600: f64, t7283: f64, t7351: f64, t7356: f64, t7392: f64) -> f64 {
    let t24629 = -0.82246703342411321825e-2_f64 * t7283 * t24564 - 0.16449340668482264365e-1_f64 * t7283 * t24568 - 0.82246703342411321825e-2_f64 * t7283 * t24571 - 0.54831135561607547884e-2_f64 * t24575 - 0.54831135561607547884e-2_f64 * t24577 + 4.0_f64 * t3487 * t7356 + 4.0_f64 * t1238 * t24582 - t24587 + 0.54831135561607547884e-2_f64 * t24589 * t24591 + 0.36554090374405031923e-2_f64 * t7283 * t24597 + 0.54831135561607547884e-2_f64 * t24589 * t24605 + 4.0_f64 * t3593 * t7356 + 0.82246703342411321825e-2_f64 * t2121 * t24612 + 0.16449340668482264365e-1_f64 * t7283 * t24617 - 2.0_f64 * t3487 * t7392 + 2.0_f64 * t7351 * t3600 - 2.0_f64 * t11613 * t2155 - 0.82246703342411321825e-2_f64 * t7283 * t24626;
    t24629
}
