//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 934/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk934(t20594: f64, t225: f64, t554: f64, t12215: f64, t1341: f64, t1363: f64, t16285: f64, t1827: f64, t19855: f64, t19940: f64, t19942: f64, t20512: f64, t20516: f64, t20556: f64, t20565: f64, t20570: f64, t3733: f64, t5235: f64, t559: f64, t6390: f64, t6422: f64) -> (f64, f64) {
    let t20595 = t20594 * t225;
    let t20596 = t20595 * t554;
    let t20599 = -35.0_f64 / 384.0_f64 * t19940 + 7.0_f64 / 384.0_f64 * t19942 - t12215 * t20512 / 4.0_f64 + 3.0_f64 / 16.0_f64 * t3733 * t20516 - t1341 * t20556 / 3072.0_f64 - t5235 * t6422 / 1024.0_f64 + t16285 * t6390 / 512.0_f64 + 5.0_f64 / 256.0_f64 * t1363 * t20565 - t1341 * t20570 / 3072.0_f64 - t19855 * t1827 / 1024.0_f64 + t20596 * t559 / 3072.0_f64;
    (t20595, t20599)
}
