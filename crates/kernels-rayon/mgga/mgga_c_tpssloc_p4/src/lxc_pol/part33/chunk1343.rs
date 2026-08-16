//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1343/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1343(t10165: f64, t1052: f64, t105840: f64, t1409: f64, t17575: f64, t21118: f64, t21510: f64, t21692: f64, t23327: f64, t23329: f64, t23330: f64, t23593: f64, t25406: f64, t25423: f64, t25429: f64, t25430: f64, t25442: f64, t28480: f64, t28499: f64, t28713: f64, t4660: f64, t5919: f64, t5943: f64, t6687: f64, t6690: f64, t6771: f64, t7624: f64, t7625: f64, t82411: f64, t88076: f64) -> f64 {
    let t105934 = -3.0_f64 * t17575 * t7625 - 18.0_f64 * t1052 * t10165 * t7624 * t5919 - 0.16449340668482264365e-1_f64 * t23327 * t23329 * t25423 * t21510 + 0.16449340668482264365e-1_f64 * t23327 * t25442 * t28499 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t88076 * t1409 * t5919 - 0.10966227112321509577e-1_f64 * t25429 * t23329 * t82411 * t105840 + 0.10966227112321509577e-1_f64 * t25429 * t23329 * t25430 * t21510 - 0.24674011002723396548e-1_f64 * t6687 * t25406 * t28480 + 6.0_f64 * t4660 * t28713 + 6.0_f64 * t6771 * t21692 - 0.21932454224643019154e-1_f64 * t6687 * t23593 * t6690 * t21118 - 0.82246703342411321826e-2_f64 * t23327 * t23329 * t23330 * t1409 * t5943;
    t105934
}
