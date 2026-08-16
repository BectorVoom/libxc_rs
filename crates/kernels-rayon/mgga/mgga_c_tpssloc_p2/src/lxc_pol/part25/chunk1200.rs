//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1200/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1200(t12021: f64, t12237: f64, t1323: f64, t1375: f64, t2085: f64, t24063: f64, t24088: f64, t24147: f64, t3882: f64, t3888: f64, t568: f64, t7213: f64, t81333: f64, t81339: f64, t81346: f64, t81350: f64, t81365: f64, t81375: f64) -> f64 {
    let t84688 = 0.29608813203268075857e0_f64 * t81333 - 0.9869604401089358619e-1_f64 * t81339 + 0.9869604401089358619e-1_f64 * t81346 - 0.46058153871750340221e0_f64 * t81350 + 3.0_f64 * t1323 * t24063 * t568 + 6.0_f64 * t3882 * t24088 + 0.9869604401089358619e-1_f64 * t81365 + 12.0_f64 * t3882 * t24147 - 18.0_f64 * t1375 * t12021 * t7213 * t3888 - 0.76763589786250567036e0_f64 * t81375 + t12237 * t2085 * t568;
    t84688
}
