//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 950/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk950(t74161: f64, t74163: f64, t70867: f64, t68491: f64, t74112: f64, t74115: f64, t74118: f64, t74122: f64, t74125: f64, t74128: f64, t74131: f64, t74137: f64, t74142: f64, t74147: f64, t74152: f64, t74154: f64, t74159: f64) -> f64 {
    let t76878 = 0.23268647941669485538e-4_f64 * t74161;
    let t76879 = 0.11634323970834742769e-3_f64 * t74163;
    let t76880 = 0.29795219925308487579e-4_f64 * t70867;
    let t76881 = 0.13139479569676025391e-5_f64 * t74112 + 0.58171619854173713846e-5_f64 * t74115 + 0.10511583655740820313e-5_f64 * t74118 - 0.10511583655740820313e-5_f64 * t74122 + 0.15767375483611230469e-5_f64 * t74125 + 0.52557918278704101564e-6_f64 * t74128 - 0.52557918278704101564e-6_f64 * t74131 - 0.81756761766873046872e-5_f64 * t68491 + 0.70077224371605468752e-6_f64 * t74137 - 0.10511583655740820313e-5_f64 * t74142 + 0.10511583655740820313e-5_f64 * t74147 + 0.35038612185802734376e-6_f64 * t74152 + 0.17451485956252114154e-4_f64 * t74154 + 0.17451485956252114154e-4_f64 * t74159 - t76878 + t76879 - t76880;
    t76881
}
