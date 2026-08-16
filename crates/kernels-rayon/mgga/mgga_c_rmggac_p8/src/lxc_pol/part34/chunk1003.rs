//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1003/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1003(t77528: f64, t1971: f64, t3351: f64, t44157: f64, t875: f64, t44183: f64, t69648: f64, t69663: f64, t69664: f64, t69666: f64, t71419: f64, t71429: f64, t75248: f64, t77514: f64, t77515: f64, t77517: f64, t77519: f64, t77520: f64, t77521: f64, t77525: f64, t884: f64) -> f64 {
    let t77529 = 0.85129199786595678796e-5_f64 * t77528;
    let t77532 = t3351 * t1971 * t875 * t44157;
    let t77533 = 0.85129199786595678796e-5_f64 * t77532;
    let t77536 = t3351 * t1971 * t875 * t44183;
    let t77537 = 0.85129199786595678796e-5_f64 * t77536;
    let t77538 = -t77514 - t77515 - t77517 - t77519 - t77520 - t77521 + t71419 - 0.40878380883436523436e-5_f64 * t69648 - t69663 + 0.24527028530061914063e-5_f64 * t69664 - 0.24527028530061914063e-5_f64 * t69666 + 0.59871208509319042821e-1_f64 * t884 * t77525 - t71429 + t75248 + t77529 + t77533 + t77537;
    t77538
}
