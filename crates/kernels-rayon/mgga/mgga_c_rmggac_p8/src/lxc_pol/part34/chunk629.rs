//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 629/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk629(t15636: f64, t15303: f64, t15307: f64, t15311: f64, t15315: f64, t15319: f64, t15323: f64, t15326: f64, t15331: f64, t15334: f64, t15337: f64, t15342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15637 = 0.68186654135613354322e-2_f64 * t15636;
    let t15638 = 0.93188427318671584245e-2_f64 * t15303;
    let t15639 = 0.15531404553111930708e-1_f64 * t15307;
    let t15640 = 0.10227998120342003148e-1_f64 * t15311;
    let t15643 = 0.40911992481368012592e-1_f64 * t15315;
    let t15644 = 0.3830813990396805546e-4_f64 * t15319;
    let t15645 = 0.1276937996798935182e-4_f64 * t15323;
    let t15646 = 0.1276937996798935182e-4_f64 * t15326;
    let t15647 = 0.58171619854173713846e-5_f64 * t15331;
    let t15648 = 0.2627895913935205078e-5_f64 * t15334;
    let t15649 = 0.87596530464506835935e-6_f64 * t15337;
    let t15650 = 0.87596530464506835935e-6_f64 * t15342;
    (t15637, t15638, t15639, t15640, t15643, t15644, t15645, t15646, t15647, t15648, t15649, t15650)
}
