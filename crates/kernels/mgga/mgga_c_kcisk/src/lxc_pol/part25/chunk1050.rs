//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1050/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1050<F: Float>(t18717: F, t2029: F, t16672: F, t10515: F, t10517: F, t16646: F, t16651: F, t16656: F, t16658: F, t16663: F, t16670: F, t16677: F, t16682: F, t16685: F, t16687: F, t16690: F, t16694: F, t16697: F, t18244: F, t1994: F, t5521: F, t7648: F) -> (F,) {
    let t18718 = t18717 * t2029;
    let t18721 = 0.15476481481481481481e-2 * t16672;
    let t18731 = -t18244 + 0.69644166666666666664e-2 * t16646 + 0.69644166666666666664e-2 * t16651 + 0.34822083333333333332e-2 * t16656 - 0.25794135802469135802e-3 * t16658 - 0.23214722222222222222e-2 * t16663 - 0.193e0 * t7648 * t5521 - 0.23214722222222222222e-2 * t16670 - 0.193e0 * t1994 * t18718 + t18721 + 0.15476481481481481481e-2 * t16677 + 0.77382407407407407407e-3 * t10515 - 0.77382407407407407406e-3 * t10517 + 0.92858888888888888886e-2 * t16682 - 0.61905925925925925924e-2 * t16685 + 0.46429444444444444443e-2 * t16687 + 0.11607361111111111111e-2 * t16690 + 0.46429444444444444443e-2 * t16694 - 0.17411041666666666666e-2 * t16697;
    (t18731,)
}
