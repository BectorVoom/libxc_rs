//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 915/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk915<F: Float>(t158: F, t16138: F, t16141: F, t16144: F, t16147: F, t16150: F, t16153: F, t16156: F, t16159: F, t16163: F, t16164: F, t16167: F, t16169: F, t16172: F, t16175: F, t16178: F, t16181: F, t16184: F, t16188: F, t165: F, t173: F, t5816: F, t5823: F, t5827: F) -> (F,) {
    let t16189 = -0.28104e-1 * t5827 * t16138 - 0.4684e-2 * t5827 * t16141 + 0.634e-2 * t5816 * t16144 + 0.21133333333333333334e-2 * t5816 * t16147 + 0.403305e-4 * t5823 * t16150 + 0.26887e-4 * t5823 * t16153 - 0.52833333333333333333e-3 * t165 * t16156 - 0.17611111111111111111e-3 * t165 * t16159 + t16163 + 0.30247875e-4 * t173 * t16164 + 0.31368166666666666666e-4 * t16167 + 0.4755e-2 * t165 * t16169 - 0.21078e-1 * t158 * t16172 - 0.10082625e-4 * t173 * t16175 + 0.403305e-4 * t173 * t16178 - 0.672175e-5 * t173 * t16181 + 0.22405833333333333333e-5 * t173 * t16184 - t16188;
    (t16189,)
}
