//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 623/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk623(t15110: f64, t15112: f64, t15114: f64, t15120: f64, t14478: f64, t14481: f64, t14484: f64, t14487: f64, t14490: f64, t14493: f64, t15101: f64, t15103: f64, t15116: f64, t15118: f64, t15122: f64, t15585: f64, t15586: f64, t15589: f64) -> f64 {
    let t15590 = 0.5177134851037310236e-2_f64 * t15110;
    let t15591 = 0.66380770525302906696e-3_f64 * t15112;
    let t15592 = 0.99571155787954360044e-3_f64 * t15114;
    let t15595 = 0.14464861606874801909e-3_f64 * t15120;
    let t15597 = t15585 - t15586 - t14478 - 0.68186654135613354322e-2_f64 * t15101 + 0.13637330827122670864e-1_f64 * t15103 + t14481 + t15589 - t15590 - t14484 + t15591 - t15592 - t14487 - 0.45360193192290319574e-3_f64 * t15116 + 0.63504270469206447404e-3_f64 * t15118 + t14490 + t15595 - 0.19286482142499735878e-3_f64 * t15122 - t14493;
    t15597
}
