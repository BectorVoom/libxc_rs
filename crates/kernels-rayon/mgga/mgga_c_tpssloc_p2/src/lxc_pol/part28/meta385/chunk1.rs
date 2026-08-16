//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1493/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1493(t1227: f64, t15643: f64, t11705: f64, t11719: f64, t11728: f64, t11734: f64, t11746: f64, t15610: f64, t15612: f64, t15617: f64, t15622: f64, t15627: f64, t15631: f64, t15637: f64, t15642: f64, t3490: f64, t3496: f64, t3506: f64, t3515: f64, t4974: f64, t4984: f64, t5019: f64) -> f64 {
    let t15645 = t1227 * t15643 / 1728.0_f64;
    let t15648 = -t11705 / 3456.0_f64 - t5019 * t3496 / 576.0_f64 + t11746 / 2304.0_f64 - t15610 - t1227 * t15612 / 2304.0_f64 - t1227 * t15617 / 768.0_f64 + t3506 * t15622 / 1536.0_f64 + t11719 * t15627 / 512.0_f64 - t11728 * t15631 / 512.0_f64 - t11734 * t4984 / 1536.0_f64 - t3515 * t15637 / 1536.0_f64 + t15642 - t15645 - t3490 * t4974 / 1152.0_f64;
    t15648
}
