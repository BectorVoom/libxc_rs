//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1251/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1251<F: Float>(t33225: F, t35453: F, t2804: F, t2807: F, t33212: F, t34065: F, t34444: F, t34480: F, t34485: F, t34594: F, t35093: F, t35101: F, t35105: F, t35133: F, t35150: F, t35153: F, t35427: F, t35431: F, t35439: F, t35446: F, t9740: F, t9995: F) -> (F, F) {
    let t35454 = t33225 * t35453;
    let t35457 = 0.15476481481481481481e-2 * t35093 + 0.40208333333333333334e-2 * t34594 * t9995 + 0.40208333333333333334e-2 * t34444 * t9995 - 0.10416666666666666667e-1 * t35427 * t2807 - 0.52083333333333333333e-2 * t35431 * t2807 + 0.23214722222222222222e-2 * t35101 + 0.11607361111111111111e-2 * t35105 - 0.34722222222222222222e-2 * t34480 - 0.50925925925925925926e-1 * t35439 * t2807 + 0.13402777777777777778e-2 * t34485 - t33212 + 0.19345601851851851852e-2 * t35133 - 0.10416666666666666667e-1 * t2804 * t35446 + 0.15476481481481481481e-2 * t34065 - 0.61905925925925925925e-2 * t35150 + 0.11607361111111111111e-2 * t35153 + 0.34722222222222222222e-2 * t9740 * t35454;
    (t35454, t35457)
}
