//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1373/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1373<F: Float>(t10005: F, t33234: F, t112506: F, t112508: F, t112510: F, t113038: F, t113040: F, t113042: F, t113181: F, t116773: F, t116779: F, t116782: F, t116787: F, t118105: F, t18681: F, t18682: F, t2804: F, t33220: F) -> (F,) {
    let t118150 = t10005 * t33234;
    let t118168 = -0.92592592592592592594e-2 * t118150 - 0.69444444444444444445e-2 * t113181 * t118105 - 0.23214722222222222222e-2 * t116773 + 0.15476481481481481481e-2 * t112506 + 0.77382407407407407407e-3 * t112508 + 0.12897067901234567901e-2 * t112510 - 0.23148148148148148148e-2 * t113038 + 0.46429444444444444443e-2 * t116779 + 0.92858888888888888886e-2 * t116782 - 0.61905925925925925924e-2 * t116787 - 0.17361111111111111111e-2 * t113040 + 0.23148148148148148148e-2 * t113042 + 0.34722222222222222222e-2 * t2804 * t18681 * t18682 * t33220;
    (t118168,)
}
