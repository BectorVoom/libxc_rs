//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 615/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk615<F: Float>(t313: F, t3732: F, t3726: F, t531: F, t3720: F, t808: F, t568: F, t836: F, t317: F, t3275: F, t3283: F, t3297: F, t3312: F, t3463: F, t3469: F, t3472: F, t3476: F, t3479: F, t3486: F, t3491: F, t3494: F, t3499: F, t3502: F, t3506: F, t797: F, t813: F, t833: F) -> (F, F, F, F, F, F, F) {
    let t3733 = t313 * t3732;
    let t3736 = t531 * t3726;
    let t3740 = t808 * t3720;
    let t3741 = t568 * t3740;
    let t3745 = t836 * t3720;
    let t3746 = t568 * t3745;
    let t3749 = t3463 + F::new(0.35750489951850426669e0) * t3733 * t317 + t3275 - t3472 + t3469 - t3476 - t3283 + t3479 - F::new(0.35750489951850426669e0) * t797 * t3736 - t3486 - F::new(0.38342925953920749677e0) * t3297 + t3494 - F::new(0.23005755572352449806e1) * t813 * t3741 - t3491 - t3499 + t3502 + F::new(0.38342925953920749677e0) * t3312 - t3506 + F::new(0.23005755572352449806e1) * t833 * t3746;
    (t3733, t3736, t3740, t3741, t3745, t3746, t3749)
}
