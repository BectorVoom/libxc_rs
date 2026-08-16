//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1047/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1047(t3788: f64, t835: f64, t1336: f64, t3795: f64, t3799: f64, t3853: f64, t12353: f64, t12356: f64, t12358: f64, t12361: f64, t12366: f64, t12371: f64, t12375: f64, t12379: f64, t1341: f64, t1363: f64, t3733: f64, t3778: f64, t3858: f64, t5246: f64) -> f64 {
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    let t12388 = t3799 * t3853;
    let t12390 = -5.0_f64 / 128.0_f64 * t1363 * t12353 - 35.0_f64 / 384.0_f64 * t12356 + 7.0_f64 / 384.0_f64 * t12358 - t1363 * t12361 / 768.0_f64 - 119.0_f64 / 4608.0_f64 * t12366 - t5246 * t12371 / 128.0_f64 + 3.0_f64 / 16.0_f64 * t3733 * t12375 - t1341 * t12379 / 3072.0_f64 - t3778 * t3858 / 1024.0_f64 - 7.0_f64 / 768.0_f64 * t12386 + 7.0_f64 / 1536.0_f64 * t12388;
    t12390
}
