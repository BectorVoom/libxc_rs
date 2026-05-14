//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1080/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1080<F: Float>(t31964: F, t31978: F, t31981: F, t31984: F, t31988: F, t31991: F, t31994: F, t32593: F, t32595: F, t32599: F, t32603: F, t32607: F, t32610: F, t32613: F, t32616: F, t1129: F, t3410: F, t3411: F, t397: F) -> (F, F) {
    let t32625 = -0.20833333333333333334e-1 * t32593 - 0.8041666666666666667e-2 * t32595 - 0.8101851851851851852e-1 * t32599 - 0.10416666666666666667e-1 * t32603 + 0.48611111111111111112e-1 * t32607 + 0.48611111111111111112e-1 * t32610 - 0.10416666666666666667e-1 * t32613 - 0.20833333333333333334e-1 * t32616 + 0.69644166666666666665e-2 * t31964 - 0.69644166666666666665e-2 * t31978 + 0.18571777777777777777e-1 * t31981 - 0.13928833333333333333e-1 * t31984 + 0.13928833333333333333e-1 * t31988 - 0.18571777777777777777e-1 * t31991 + 0.21667074074074074073e-1 * t31994;
    let t32628 = t397 * t3410 * t1129 * t3411;
    (t32625, t32628)
}
