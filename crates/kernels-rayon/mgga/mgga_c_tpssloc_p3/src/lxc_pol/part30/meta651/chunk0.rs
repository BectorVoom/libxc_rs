//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2065/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2065(t1598: f64, t974: f64, t23631: f64, t1920: f64, t25535: f64, t968: f64, t23665: f64, t25479: f64, t25487: f64, t82736: f64, t25493: f64, t7611: f64, t82713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89242 = t974 * t1598;
    let t89243 = t23631 * t89242;
    let t89256 = 0.54831135561607547884e-2_f64 * t1920 * t968 * t25535;
    let t89292 = 0.54831135561607547884e-2_f64 * t23665 * t25479;
    let t89294 = 0.10966227112321509577e-1_f64 * t82736 * t25487;
    let t89296 = 0.54831135561607547884e-2_f64 * t82736 * t25493;
    let t89309 = 0.14621636149762012769e-1_f64 * t82713 * t7611;
    (t89242, t89243, t89256, t89292, t89294, t89296, t89309)
}
