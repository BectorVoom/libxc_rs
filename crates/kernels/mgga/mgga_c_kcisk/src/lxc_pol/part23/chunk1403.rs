//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1403/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1403<F: Float>(t113511: F, t113513: F, t113515: F, t113517: F, t113519: F, t113521: F, t113523: F, t113525: F, t113527: F, t113529: F, t113531: F, t41204: F, t9831: F, t1610: F, t33975: F, t14294: F, t1520: F, t33633: F) -> (F, F, F, F) {
    let t114956 = 0.61111111111111111112e0 * t113511 - 0.20234375e-1 * t113513 - 0.125e0 * t113515 + 0.28777777777777777778e0 * t113517 + 0.33333333333333333334e0 * t113519 - 0.125e0 * t113521 + 0.20234375e-1 * t113523 - 0.89930555555555555557e-2 * t113525 + 0.53958333333333333334e-1 * t113527 - 0.5e0 * t113529 - 0.91666666666666666667e0 * t113531;
    let t114962 = 2.0 * t41204 * t9831;
    let t114965 = t33975 * t1610;
    let t114970 = 12.0 * t14294 * t33633 * t1520;
    (t114956, t114962, t114965, t114970)
}
