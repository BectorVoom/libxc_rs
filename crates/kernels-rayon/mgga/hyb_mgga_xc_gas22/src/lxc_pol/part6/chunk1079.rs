//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1079/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1079(t1240: f64, t3279: f64, t3985: f64, t550: f64, t136: f64, t3990: f64, t680: f64, t3946: f64, t1319: f64, t2950: f64, t10446: f64, t10450: f64, t10457: f64, t10461: f64, t10463: f64, t216: f64, t2949: f64, t3274: f64, t3288: f64, t3986: f64, t677: f64, t766: f64) -> (f64, f64, f64, f64) {
    let t10465 = t1240 * t3279;
    let t10467 = t550 * t3985;
    let t10468 = t136 * t10467;
    let t10470 = t3990 * t680;
    let t10472 = t550 * t3946;
    let t10473 = t136 * t10472;
    let t10477 = t2950 * t1319;
    let t10480 = -3.0_f64 / 64.0_f64 * t677 * t3986 - 3.0_f64 / 64.0_f64 * t136 * t10446 - 3.0_f64 / 64.0_f64 * t10450 * t216 - 3.0_f64 / 64.0_f64 * t3990 * t766 - 3.0_f64 / 32.0_f64 * t1240 * t3274 + 3.0_f64 / 16.0_f64 * t2949 * t10457 - t10461 / 64.0_f64 - t10463 / 32.0_f64 - t10465 / 32.0_f64 - t10468 / 64.0_f64 - t10470 / 64.0_f64 - t10473 / 32.0_f64 - 3.0_f64 / 32.0_f64 * t1240 * t3288 + 3.0_f64 / 16.0_f64 * t2949 * t10477;
    (t10467, t10472, t10477, t10480)
}
