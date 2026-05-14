//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1346/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1346<F: Float>(t117390: F, t117392: F, t117394: F, t117396: F, t117398: F, t117401: F, t117403: F, t117405: F, t117407: F, t117411: F, t117413: F, t117415: F, t117417: F, t117420: F, t117422: F, t117424: F, t117427: F, t117429: F) -> (F,) {
    let t117518 = -0.809375e-1 * t117390 - 0.809375e-1 * t117392 + 0.20833333333333333333e-1 * t117394 - 0.20234375e-1 * t117396 + 0.10791666666666666667e0 * t117398 + 0.5e0 * t117401 + 0.53958333333333333334e-1 * t117403 + 0.53958333333333333334e-1 * t117405 + 0.26979166666666666667e-1 * t117407 - 0.28777777777777777778e0 * t117411 + 0.1875e0 * t117413 + 0.59953703703703703705e-2 * t117415 - 0.125e0 * t117417 + 0.53958333333333333334e-1 * t117420 + 0.89930555555555555557e-2 * t117422 - 0.89930555555555555557e-2 * t117424 + 0.41666666666666666666e-1 * t117427 - 0.53958333333333333334e-1 * t117429;
    (t117518,)
}
