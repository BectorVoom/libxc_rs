//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1421/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1421<F: Float>(t1586: F, t2805: F, t73204: F, t35462: F, t9724: F, t10000: F, t10014: F, t118326: F, t121265: F, t121269: F, t121272: F, t121275: F, t34452: F, t34457: F, t34462: F, t34469: F, t35439: F, t35476: F, t9721: F, t9725: F, t9728: F, t9748: F, t9991: F, t9995: F) -> (F, F) {
    let t122638 = t1586 * t2805 * t73204;
    let t122643 = t9724 * t35462;
    let t122646 = 0.12897067901234567901e-2 * t121265 + 0.19345601851851851852e-2 * t121269 + 0.10416666666666666667e-1 * t10000 * t34469 - 0.11607361111111111111e-1 * t121272 + 0.92858888888888888888e-2 * t121275 + 0.50925925925925925926e-1 * t35439 * t9748 + 0.50925925925925925926e-1 * t35439 * t9728 + 0.10416666666666666667e-1 * t34452 * t9995 + 0.10416666666666666667e-1 * t34457 * t9995 + 0.10416666666666666667e-1 * t9991 * t34469 + 0.52083333333333333333e-2 * t9721 * t35476 + 0.40208333333333333334e-2 * t118326 * t9995 + 0.20104166666666666667e-2 * t9725 * t122638 - 0.27777777777777777778e-1 * t34462 * t10014 + 0.20104166666666666667e-2 * t122643 * t9728;
    (t122638, t122646)
}
