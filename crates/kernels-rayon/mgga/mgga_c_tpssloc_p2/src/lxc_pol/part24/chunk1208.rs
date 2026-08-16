//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1208/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1208(t23661: f64, t3201: f64, t1058: f64, t1920: f64, t1950: f64, t23323: f64, t23327: f64, t23601: f64, t23606: f64, t23610: f64, t23614: f64, t23619: f64, t23621: f64, t23626: f64, t23629: f64, t23633: f64, t23637: f64, t23642: f64, t23644: f64, t23647: f64, t23650: f64, t23654: f64, t23658: f64, t3180: f64, t3200: f64, t6687: f64, t6797: f64, t6811: f64) -> (f64, f64) {
    let t23662 = t23661 * t3201;
    let t23664 = -0.82246703342411321825e-2_f64 * t23601 * t23606 + 0.16449340668482264365e-1_f64 * t6797 * t23610 - 0.54831135561607547884e-2_f64 * t23327 * t23614 - t23619 + 0.82246703342411321825e-2_f64 * t1920 * t23621 + 0.80418998823691070228e-1_f64 * t23323 * t1950 - 0.14621636149762012769e-1_f64 * t23626 + 0.54831135561607547884e-2_f64 * t23629 + 0.54831135561607547884e-2_f64 * t23633 * t23637 + 2.0_f64 * t3180 * t6811 - 0.54831135561607547884e-2_f64 * t23642 - 0.82246703342411321825e-2_f64 * t6687 * t23644 - 0.16449340668482264365e-1_f64 * t6687 * t23647 - 0.82246703342411321825e-2_f64 * t6687 * t23650 + 2.0_f64 * t1058 * t23654 - 0.16449340668482264365e-1_f64 * t6797 * t23658 - t3200 * t23662;
    (t23662, t23664)
}
