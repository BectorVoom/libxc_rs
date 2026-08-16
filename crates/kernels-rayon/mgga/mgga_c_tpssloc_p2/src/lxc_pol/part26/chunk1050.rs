//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1050/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1050(t3777: f64, t3802: f64, t12392: f64, t12395: f64, t12397: f64, t12404: f64, t12409: f64, t12413: f64, t12422: f64, t12426: f64, t1341: f64, t1354: f64, t3778: f64, t3783: f64, t3803: f64, t3809: f64, t3853: f64, t3872: f64) -> f64 {
    let t12429 = t3777 * t3802;
    let t12432 = -t1341 * t12392 / 3072.0_f64 + 7.0_f64 / 1536.0_f64 * t12395 - t12397 * t1354 / 1024.0_f64 - t3778 * t3853 / 1024.0_f64 + t3803 * t12404 / 256.0_f64 + t3803 * t12409 / 256.0_f64 - t3803 * t12413 / 1024.0_f64 + 5.0_f64 / 256.0_f64 * t3783 * t3872 - 5.0_f64 / 256.0_f64 * t3803 * t12422 + t3803 * t12426 / 256.0_f64 + t12429 * t3809 / 128.0_f64;
    t12432
}
