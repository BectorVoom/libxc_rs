//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 630/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk630(t15345: f64, t15348: f64, t15351: f64, t15354: f64, t15357: f64, t15359: f64, t15364: f64, t15368: f64, t15372: f64, t15377: f64, t15380: f64, t15389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15651 = 0.17519306092901367188e-6_f64 * t15345;
    let t15652 = 0.4379826523225341797e-6_f64 * t15348;
    let t15653 = 0.35038612185802734376e-6_f64 * t15351;
    let t15654 = 0.52557918278704101564e-6_f64 * t15354;
    let t15655 = 0.52557918278704101564e-6_f64 * t15357;
    let t15656 = 0.14967802127329760705e-1_f64 * t15359;
    let t15657 = 0.58171619854173713846e-5_f64 * t15364;
    let t15658 = 0.17451485956252114154e-4_f64 * t15368;
    let t15660 = 0.23268647941669485538e-4_f64 * t15372;
    let t15661 = 0.58171619854173713846e-5_f64 * t15377;
    let t15662 = 0.58171619854173713846e-5_f64 * t15380;
    let t15663 = 0.35038612185802734376e-6_f64 * t15389;
    (t15651, t15652, t15653, t15654, t15655, t15656, t15657, t15658, t15660, t15661, t15662, t15663)
}
