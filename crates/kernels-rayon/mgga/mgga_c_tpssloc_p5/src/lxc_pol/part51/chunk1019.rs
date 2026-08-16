//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1019/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1019(t25510: f64, t25512: f64, t1625: f64, t362: f64, t884: f64, t6784: f64, t6743: f64, t7577: f64, t6801: f64, t1058: f64, t23327: f64, t23601: f64, t23642: f64, t23670: f64, t25487: f64, t25493: f64, t25497: f64, t25500: f64, t25503: f64, t25508: f64, t3180: f64, t6687: f64, t6797: f64, t7611: f64, t7620: f64) -> f64 {
    let t25513 = t25510 * t25512;
    let t25516 = t362 * t1625;
    let t25517 = t25516 * t884;
    let t25518 = t6784 * t25517;
    let t25523 = t7577 * t6743;
    let t25524 = t25523 * t6801;
    let t25527 = 0.16449340668482264365e-1_f64 * t23601 * t25487 - 0.82246703342411321825e-2_f64 * t23601 * t25493 + t1058 * t25497 + t1058 * t25500 + 0.82246703342411321825e-2_f64 * t6797 * t25503 - 0.21932454224643019153e-1_f64 * t23670 * t7611 + 0.27415567780803773942e-2_f64 * t25508 - 0.54831135561607547884e-2_f64 * t23327 * t25513 + 0.27415567780803773942e-2_f64 * t6687 * t25518 - 0.27415567780803773942e-2_f64 * t23642 + t3180 * t7620 - 0.82246703342411321825e-2_f64 * t6797 * t25524;
    t25527
}
