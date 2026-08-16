//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1344/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1344(t1052: f64, t1599: f64, t1634: f64, t18074: f64, t1955: f64, t21662: f64, t21676: f64, t21691: f64, t23327: f64, t23329: f64, t23330: f64, t23394: f64, t25429: f64, t25442: f64, t25755: f64, t25810: f64, t28485: f64, t28491: f64, t28499: f64, t28679: f64, t3174: f64, t4660: f64, t5398: f64, t5944: f64, t6687: f64, t6704: f64, t7625: f64, t82481: f64, t99301: f64, t99330: f64, t99400: f64) -> f64 {
    let t105971 = -0.54831135561607547883e-2_f64 * t99301 - 0.16449340668482264365e-1_f64 * t6687 * t25810 * t28499 + 0.49348022005446793095e-1_f64 * t6687 * t1599 * t99400 + 12.0_f64 * t4660 * t28485 + 0.49348022005446793095e-1_f64 * t6687 * t6704 * t23394 * t21691 + 2.0_f64 * t1052 * t3174 * t1955 * t21662 - 3.0_f64 * t4660 * t28679 + 0.54831135561607547883e-2_f64 * t99330 - 0.10966227112321509577e-1_f64 * t25429 * t25442 * t28491 - 0.82246703342411321826e-2_f64 * t23327 * t23329 * t23330 * t5398 * t1634 - 3.0_f64 * t25755 * t5944 - 0.49348022005446793095e-1_f64 * t6687 * t6704 * t82481 * t21676 - 3.0_f64 * t18074 * t7625;
    t105971
}
