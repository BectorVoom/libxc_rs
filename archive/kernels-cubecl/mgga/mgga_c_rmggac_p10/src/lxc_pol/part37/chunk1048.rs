//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1048/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1048<F: Float>(t74107: F, t68491: F, t74105: F, t74112: F, t74115: F, t74118: F, t74122: F, t74125: F, t74128: F, t74131: F, t74137: F, t74142: F, t74147: F, t74152: F, t74154: F, t74159: F, t76859: F) -> F {
    let t80037 = F::cast_from(0.36357262408858571152e-4_f64) * t74107;
    let t80052 = t76859 + F::cast_from(0.58171619854173713844e-5_f64) * t74105 + t80037 + F::cast_from(0.1313947956967602539e-5_f64) * t74112 + F::cast_from(0.58171619854173713844e-5_f64) * t74115 + F::cast_from(0.10511583655740820312e-5_f64) * t74118 - F::cast_from(0.10511583655740820312e-5_f64) * t74122 + F::cast_from(0.15767375483611230468e-5_f64) * t74125 + F::cast_from(0.52557918278704101561e-6_f64) * t74128 - F::cast_from(0.52557918278704101561e-6_f64) * t74131 - F::cast_from(0.8175676176687304687e-5_f64) * t68491 + F::cast_from(0.70077224371605468748e-6_f64) * t74137 - F::cast_from(0.10511583655740820312e-5_f64) * t74142 + F::cast_from(0.10511583655740820312e-5_f64) * t74147 + F::cast_from(0.35038612185802734374e-6_f64) * t74152 + F::cast_from(0.17451485956252114153e-4_f64) * t74154 + F::cast_from(0.17451485956252114153e-4_f64) * t74159;
    t80052
}
