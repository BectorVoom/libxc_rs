//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 626/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk626(t14980: f64, t570: f64, t14478: f64, t14481: f64, t14484: f64, t14487: f64, t14493: f64, t14933: f64, t15101: f64, t15103: f64, t15116: f64, t15118: f64, t15122: f64, t15585: f64, t15586: f64, t15589: f64, t15590: f64, t15591: f64, t15592: f64, t15595: f64) -> (f64, f64) {
    let t15872 = t14980 * t570;
    let t15881 = t15585 - t15586 - t14478 - 0.68186654135613354324e-2_f64 * t15101 + 0.13637330827122670865e-1_f64 * t15103 + t14481 + t15589 - t15590 - t14484 + t15591 - t15592 - t14487 - 0.45360193192290319575e-3_f64 * t15116 + 0.63504270469206447405e-3_f64 * t15118 + t14933 + t15595 - 0.19286482142499735879e-3_f64 * t15122 - t14493;
    (t15872, t15881)
}
