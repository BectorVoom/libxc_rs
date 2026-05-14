//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 783/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk783<F: Float>(t10000: F, t10005: F, t10009: F, t10014: F, t2804: F, t2807: F, t9725: F, t9738: F, t9740: F, t9753: F, t9947: F, t9950: F, t9953: F, t9958: F, t9961: F, t9991: F, t9995: F) -> (F,) {
    let t10024 = -0.52083333333333333333e-2 * t9991 * t2807 + 0.20104166666666666667e-2 * t9725 * t9995 - 0.52083333333333333333e-2 * t10000 * t2807 + 0.13888888888888888889e-1 * t10005 * t2807 - t9738 - 0.17361111111111111111e-2 * t9740 * t10009 + 0.52083333333333333333e-2 * t2804 * t10014 + 0.52083333333333333333e-2 * t2804 * t9995 + t9753 + 0.11607361111111111111e-2 * t9947 + 0.17411041666666666666e-2 * t9950 - 0.17411041666666666666e-2 * t9953 - 0.46429444444444444443e-2 * t9958 + 0.11607361111111111111e-2 * t9961;
    (t10024,)
}
