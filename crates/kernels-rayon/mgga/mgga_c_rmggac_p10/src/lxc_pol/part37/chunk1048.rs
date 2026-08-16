//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1048/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1048(t74107: f64, t68491: f64, t74105: f64, t74112: f64, t74115: f64, t74118: f64, t74122: f64, t74125: f64, t74128: f64, t74131: f64, t74137: f64, t74142: f64, t74147: f64, t74152: f64, t74154: f64, t74159: f64, t76859: f64) -> f64 {
    let t80037 = 0.36357262408858571152e-4_f64 * t74107;
    let t80052 = t76859 + 0.58171619854173713844e-5_f64 * t74105 + t80037 + 0.1313947956967602539e-5_f64 * t74112 + 0.58171619854173713844e-5_f64 * t74115 + 0.10511583655740820312e-5_f64 * t74118 - 0.10511583655740820312e-5_f64 * t74122 + 0.15767375483611230468e-5_f64 * t74125 + 0.52557918278704101561e-6_f64 * t74128 - 0.52557918278704101561e-6_f64 * t74131 - 0.8175676176687304687e-5_f64 * t68491 + 0.70077224371605468748e-6_f64 * t74137 - 0.10511583655740820312e-5_f64 * t74142 + 0.10511583655740820312e-5_f64 * t74147 + 0.35038612185802734374e-6_f64 * t74152 + 0.17451485956252114153e-4_f64 * t74154 + 0.17451485956252114153e-4_f64 * t74159;
    t80052
}
