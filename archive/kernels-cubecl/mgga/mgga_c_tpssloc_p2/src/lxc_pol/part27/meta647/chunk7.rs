//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2237/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2237<F: Float>(t23665: F, t25479: F, t25487: F, t82736: F, t25493: F, t23633: F, t23696: F, t25516: F, t25553: F, t25554: F, t25568: F, t2771: F, t2776: F, t3180: F, t4542: F, t4688: F, t6687: F, t6743: F, t6800: F, t6805: F, t7611: F, t82527: F, t82734: F, t82737: F, t82739: F) -> F {
    let t89292 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25479;
    let t89294 = F::cast_from(0.10966227112321509577e-1_f64) * t82736 * t25487;
    let t89296 = F::cast_from(0.54831135561607547884e-2_f64) * t82736 * t25493;
    let t89297 = -F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t25553 * t6800 * t2776 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t6743 * t4688 * t25554 + F::cast_from(2.0_f64) * t3180 * t25568 + F::cast_from(0.27415567780803773942e-2_f64) * t82734 + F::cast_from(0.54831135561607547884e-2_f64) * t82737 - F::cast_from(0.27415567780803773942e-2_f64) * t82739 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t6805 + F::cast_from(0.36554090374405031923e-2_f64) * t6687 * t23696 * t25516 * t2771 + F::cast_from(0.80418998823691070228e-1_f64) * t82527 * t7611 - t89292 + t89294 - t89296;
    t89297
}
