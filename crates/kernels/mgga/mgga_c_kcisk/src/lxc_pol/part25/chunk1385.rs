//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1385/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1385<F: Float>(t117193: F, t117195: F, t117203: F, t117207: F, t112674: F, t112683: F, t112696: F, t112876: F, t113162: F, t117184: F, t117205: F, t117213: F, t117931: F, t18401: F, t33220: F, t9740: F) -> (F,) {
    let t118493 = 0.15476481481481481481e-2 * t117193;
    let t118494 = 0.15476481481481481481e-2 * t117195;
    let t118495 = 0.23214722222222222222e-2 * t117203;
    let t118497 = 0.15476481481481481481e-2 * t117207;
    let t118506 = -0.15476481481481481481e-2 * t112674 + 0.11574074074074074074e-2 * t113162 + 0.46429444444444444443e-2 * t117184 - 0.11607361111111111111e-2 * t112683 + t118493 + t118494 + t118495 + 0.46429444444444444443e-2 * t117205 - t118497 + 0.15476481481481481481e-2 * t112696 + 0.15476481481481481481e-2 * t117213 - 0.34722222222222222222e-2 * t9740 * t112876 * t33220 * t18401 - 0.34722222222222222222e-2 * t9740 * t117931;
    (t118506,)
}
