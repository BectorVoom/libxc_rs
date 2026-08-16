//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2287/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287(t27438: f64, t85639: f64, t225: f64, t27419: f64, t1236: f64, t1252: f64, t12652: f64, t1409: f64, t15797: f64, t15820: f64, t2128: f64, t24589: f64, t24590: f64, t24601: f64, t24602: f64, t24626: f64, t24638: f64, t24877: f64, t254: f64, t27388: f64, t27406: f64, t27444: f64, t27747: f64, t27786: f64, t3487: f64, t3630: f64, t4936: f64, t4945: f64, t7356: f64, t7392: f64) -> f64 {
    let t94648 = 0.18277045187202515961e-2_f64 * t85639 * t27438;
    let t94656 = t27419 * t225;
    let t94673 = -2.0_f64 * t15797 * t7392 + 4.0_f64 * t3487 * t27747 - 2.0_f64 * t15820 * t7392 - 12.0_f64 * t1236 * t254 * t27786 + t94648 + 0.54831135561607547884e-2_f64 * t24589 * t24590 * t27388 - 0.10966227112321509577e-1_f64 * t24589 * t24601 * t27444 * t12652 - 2.0_f64 * t94656 * t1252 + 4.0_f64 * t15797 * t7356 + 2.0_f64 * t4945 * t24877 + 0.16449340668482264365e-1_f64 * t2128 * t4936 * t24638 + 0.27415567780803773942e-2_f64 * t24589 * t24601 * t24602 * t1409 * t3630 + 0.21932454224643019153e-1_f64 * t27406 * t24626;
    t94673
}
