//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1040/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1040(t12020: f64, t8800: f64, t115397: f64, t115409: f64, t115415: f64, t115423: f64, t115430: f64, t117193: f64, t117209: f64, t117210: f64, t117246: f64, t122451: f64, t122457: f64, t122460: f64, t122462: f64, t122467: f64, t122483: f64, t122488: f64, t1336: f64, t1814: f64, t1825: f64, t32137: f64, t32148: f64, t33839: f64, t3777: f64, t5234: f64) -> (f64, f64) {
    let t124223 = t12020 * t8800;
    let t124245 = 0.19739208802178717238e0_f64 * t122451 - 0.3289868133696452873e-1_f64 * t122457 + 0.16449340668482264365e-1_f64 * t122460 + 0.76763589786250567037e-1_f64 * t122462 - 0.3289868133696452873e-1_f64 * t122467 - t117193 + 0.15352717957250113407e0_f64 * t115397 + 0.3289868133696452873e-1_f64 * t115409 + 0.3289868133696452873e-1_f64 * t122483 + 0.76763589786250567037e-1_f64 * t115415 - 0.3289868133696452873e-1_f64 * t122488 + t1814 * t32148 + 0.16449340668482264365e-1_f64 * t115423 - 0.76763589786250567037e-1_f64 * t115430 + t117209 + t117210 - t5234 * t32137 - t3777 * t33839 - t1336 * t117246 * t1825;
    (t124223, t124245)
}
