//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1366/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1366<F: Float>(t33257: F, t9999: F, t10014: F, t112925: F, t116495: F, t116498: F, t116507: F, t117663: F, t117969: F, t1586: F, t18674: F, t2021: F, t2029: F, t2804: F, t33196: F, t33284: F, t33287: F, t33291: F, t34520: F, t9721: F, t9725: F, t9728: F, t9733: F, t9995: F) -> (F,) {
    let t118003 = t33257 * t9999;
    let t118009 = 0.20104166666666666667e-2 * t9725 * t117969 + 0.77160493827160493826e-3 * t112925 + 0.10416666666666666667e-1 * t33287 * t9995 + 0.52083333333333333333e-2 * t33291 * t9995 + 0.52083333333333333333e-2 * t2804 * t1586 * t2021 * t2029 * t18674 + 0.10416666666666666667e-1 * t9733 * t34520 - 0.46429444444444444444e-2 * t116495 + 0.38691203703703703704e-2 * t116498 + 0.52083333333333333333e-2 * t33284 * t10014 + 0.10416666666666666667e-1 * t9721 * t34520 + 0.40208333333333333334e-2 * t118003 * t9728 - 0.40208333333333333334e-2 * t33196 * t117663 - 0.15476481481481481481e-2 * t116507;
    (t118009,)
}
