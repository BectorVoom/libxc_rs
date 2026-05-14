//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1444/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1444<F: Float>(t10009: F, t117694: F, t117791: F, t122041: F, t122044: F, t122047: F, t122050: F, t122054: F, t122060: F, t122063: F, t123079: F, t1586: F, t2021: F, t2029: F, t25101: F, t2804: F, t33176: F, t34395: F, t34400: F, t34406: F, t34415: F, t34416: F, t9740: F) -> (F,) {
    let t123285 = 0.52083333333333333333e-2 * t2804 * t1586 * t2021 * t2029 * t25101 - 0.23280625e-2 * t33176 * t34415 * t34406 - 0.10416666666666666667e-1 * t34416 * t34400 - 0.20833333333333333334e-1 * t34416 * t34406 - 0.34722222222222222222e-2 * t117791 * t10009 - 0.34722222222222222222e-2 * t117694 * t10009 + 0.69444444444444444444e-2 * t34416 * t34395 - 0.52083333333333333333e-2 * t9740 * t123079 - 0.17411041666666666666e-2 * t122041 - 0.61905925925925925925e-2 * t122044 + 0.11607361111111111111e-2 * t122047 + 0.11607361111111111111e-2 * t122050 + 0.61905925925925925924e-2 * t122054 - 0.17024129629629629629e-1 * t122060 + 0.11349419753086419753e-1 * t122063;
    (t123285,)
}
