//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1249/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1249<F: Float>(t10000: F, t10009: F, t2807: F, t33196: F, t34390: F, t34412: F, t34416: F, t34435: F, t34449: F, t34466: F, t35082: F, t35086: F, t35090: F, t35383: F, t35389: F, t35395: F, t35402: F, t35410: F, t35416: F, t9725: F, t9740: F, t9995: F) -> (F,) {
    let t35421 = -0.11574074074074074074e-2 * t34390 + 0.34722222222222222222e-2 * t9740 * t35383 - 0.34722222222222222222e-2 * t34435 * t10009 - 0.23148148148148148148e-2 * t9740 * t35389 + 0.92592592592592592593e-2 * t34412 * t10009 - 0.17361111111111111111e-2 * t9740 * t35395 - 0.34722222222222222222e-2 * t34416 * t10009 - 0.40208333333333333334e-2 * t33196 * t35402 - 0.61905925925925925925e-2 * t35082 - 0.23214722222222222222e-2 * t35086 + 0.92592592592592592593e-2 * t34449 + 0.27777777777777777778e-1 * t35410 * t2807 + 0.10416666666666666667e-1 * t10000 * t9995 - 0.60312500000000000001e-2 * t9725 * t35416 + 0.34722222222222222222e-2 * t34466 - 0.23214722222222222222e-2 * t35090;
    (t35421,)
}
