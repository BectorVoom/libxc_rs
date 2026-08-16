//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 421/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk421(t313: f64, t3732: f64, t3726: f64, t531: f64, t3720: f64, t808: f64, t568: f64, t836: f64, t317: f64, t3275: f64, t3283: f64, t3297: f64, t3312: f64, t3463: f64, t3469: f64, t3472: f64, t3476: f64, t3479: f64, t3486: f64, t3491: f64, t3494: f64, t3499: f64, t3502: f64, t3506: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3733 = t313 * t3732;
    let t3736 = t531 * t3726;
    let t3740 = t808 * t3720;
    let t3741 = t568 * t3740;
    let t3745 = t836 * t3720;
    let t3746 = t568 * t3745;
    let t3749 = t3463 + 0.35750489951850426669e0_f64 * t3733 * t317 + t3275 - t3472 + t3469 - t3476 - t3283 + t3479 - 0.35750489951850426669e0_f64 * t797 * t3736 - t3486 - 0.38342925953920749677e0_f64 * t3297 + t3494 - 0.23005755572352449806e1_f64 * t813 * t3741 - t3491 - t3499 + t3502 + 0.38342925953920749677e0_f64 * t3312 - t3506 + 0.23005755572352449806e1_f64 * t833 * t3746;
    (t3733, t3736, t3740, t3741, t3745, t3746, t3749)
}
