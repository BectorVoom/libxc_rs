//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2237/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237(t23665: f64, t25479: f64, t25487: f64, t82736: f64, t25493: f64, t23633: f64, t23696: f64, t25516: f64, t25553: f64, t25554: f64, t25568: f64, t2771: f64, t2776: f64, t3180: f64, t4542: f64, t4688: f64, t6687: f64, t6743: f64, t6800: f64, t6805: f64, t7611: f64, t82527: f64, t82734: f64, t82737: f64, t82739: f64) -> f64 {
    let t89292 = 0.54831135561607547884e-2_f64 * t23665 * t25479;
    let t89294 = 0.10966227112321509577e-1_f64 * t82736 * t25487;
    let t89296 = 0.54831135561607547884e-2_f64 * t82736 * t25493;
    let t89297 = -0.54831135561607547884e-2_f64 * t23633 * t25553 * t6800 * t2776 + 0.54831135561607547884e-2_f64 * t23633 * t6743 * t4688 * t25554 + 2.0_f64 * t3180 * t25568 + 0.27415567780803773942e-2_f64 * t82734 + 0.54831135561607547884e-2_f64 * t82737 - 0.27415567780803773942e-2_f64 * t82739 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t6805 + 0.36554090374405031923e-2_f64 * t6687 * t23696 * t25516 * t2771 + 0.80418998823691070228e-1_f64 * t82527 * t7611 - t89292 + t89294 - t89296;
    t89297
}
