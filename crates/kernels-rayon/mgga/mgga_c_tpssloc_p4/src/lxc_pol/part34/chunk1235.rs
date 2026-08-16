//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1235/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1235(t101355: f64, t101593: f64, t105223: f64, t105232: f64, t105240: f64, t105445: f64, t105449: f64, t105453: f64, t108342: f64, t108361: f64, t108378: f64, t108412: f64, t108430: f64, t108448: f64, t1528: f64, t2053: f64, t2054: f64, t21049: f64, t26700: f64, t26713: f64, t29060: f64, t29080: f64, t40890: f64, t4147: f64, t5637: f64, t68322: f64, t85101: f64, t855: f64, t86870: f64, t86903: f64, t86911: f64, t98117: f64, t98322: f64) -> f64 {
    let t108451 = 12.0_f64 * t4147 * t29080 - t68322 * t2054 - 3.0_f64 * t101355 * t1528 + 6.0_f64 * t4147 * t29060 - 3.0_f64 * t101593 * t1528 + 6.0_f64 * t26713 * t5637 + 6.0_f64 * t26700 * t5637 + 0.15626873635058151147e0_f64 * t86911 - 0.76763589786250567036e0_f64 * t86903 - 0.31253747270116302294e0_f64 * t86870 - t85101 + t108342 + 0.29608813203268075857e0_f64 * t105449 + 0.46058153871750340221e0_f64 * t98117 + t108378 + t108448 + t108430 - 0.9869604401089358619e-1_f64 * t105445 + t108361 - 0.3289868133696452873e-1_f64 * t105453 + 0.29608813203268075857e0_f64 * t105223 + 0.9869604401089358619e-1_f64 * t105240 - 0.39478417604357434476e0_f64 * t105232 + t108412 + 0.49348022005446793095e-1_f64 * t98322 + 24.0_f64 * t855 * t40890 * t2053 * t21049;
    t108451
}
