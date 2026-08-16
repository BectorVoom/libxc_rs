//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2298/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2298(t1920: f64, t28474: f64, t968: f64, t14529: f64, t14555: f64, t1599: f64, t17187: f64, t1956: f64, t23372: f64, t25766: f64, t28485: f64, t3026: f64, t4542: f64, t5920: f64, t61061: f64, t6687: f64, t6689: f64, t6690: f64, t7561: f64, t7600: f64, t7625: f64, t88182: f64, t89561: f64, t89583: f64, t89597: f64) -> f64 {
    let t99877 = t1920 * t968 * t28474;
    let t99894 = t89561 - t61061 * t1956 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t7561 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t25766 + 0.27415567780803773942e-2_f64 * t99877 + t89583 + 2.0_f64 * t23372 * t5920 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t88182 + 0.27415567780803773942e-2_f64 * t6687 * t6689 * t6690 * t17187 + 4.0_f64 * t3026 * t28485 - 2.0_f64 * t14555 * t7625 - t89597 + 4.0_f64 * t14529 * t7600;
    t99894
}
