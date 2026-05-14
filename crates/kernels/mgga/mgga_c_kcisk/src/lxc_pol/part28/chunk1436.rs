//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1436/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1436<F: Float>(t34415: F, t9990: F, t112982: F, t2028: F, t7261: F, t9176: F, t2647: F, t33197: F, t7638: F, t2023: F, t9234: F, t117008: F, t118348: F, t121765: F, t121767: F, t121774: F, t121777: F, t2642: F, t33196: F, t33208: F, t33297: F, t34406: F, t34422: F, t34435: F, t35402: F, t9740: F, t9743: F) -> (F, F, F) {
    let t123051 = t9990 * t34415;
    let t123056 = t7261 * t112982 * t9176 * t2028;
    let t123072 = t7261 * t33197 * t7638 * t2647;
    let t123079 = t7261 * t33197 * t9234 * t2023;
    let t123082 = 0.17411041666666666666e-2 * t121765 + 0.15476481481481481481e-2 * t121767 - t118348 - 0.41270617283950617283e-2 * t121774 - 0.34822083333333333332e-2 * t121777 + 0.77382407407407407407e-3 * t117008 - 0.34722222222222222223e-2 * t123051 * t9743 + 0.10416666666666666667e-1 * t9740 * t123056 - 0.20833333333333333334e-1 * t9740 * t7261 * t34422 * t2642 * t7638 + 0.40208333333333333335e-2 * t33196 * t123056 - 0.10416666666666666667e-1 * t33297 * t35402 - 0.10416666666666666667e-1 * t33208 * t35402 - 0.10416666666666666667e-1 * t9740 * t123072 - 0.20833333333333333334e-1 * t34435 * t34406 - 0.20104166666666666667e-2 * t33196 * t123079;
    (t123072, t123079, t123082)
}
