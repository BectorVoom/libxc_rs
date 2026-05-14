//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1183/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1183<F: Float>(t1394: F, t27364: F, t6904: F, t22271: F, t5780: F, t7923: F, t20975: F, t27387: F, t20980: F, t20985: F, t21894: F, t1014: F, t29340: F, t29383: F, t102328: F, t18128: F, t27583: F, t27584: F, t28701: F, t77753: F, t7978: F, t99248: F, t99646: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102698 = t1394 * t27364 * t6904;
    let t102701 = t5780 * t7923 * t22271;
    let t102706 = t1394 * t27387 * t20975;
    let t102709 = t1394 * t7923 * t20980;
    let t102712 = t1394 * t7923 * t20985;
    let t102715 = t5780 * t7923 * t21894;
    let t102723 = t1014 * t29340;
    let t102725 = t1014 * t29383;
    let t102727 = 0.15476481481481481481e-2 * t102698 - 0.30952962962962962962e-2 * t102701 + 0.69505208333333333334e-3 * t7978 * t102328 + 0.23214722222222222222e-2 * t102706 - 0.15476481481481481481e-2 * t102709 + 0.69644166666666666666e-2 * t102712 + 0.92858888888888888888e-2 * t102715 - 0.46336805555555555556e-3 * t27583 * t18128 * t27584 * t77753 - 0.82448622685185185187e-4 * t99248 * t28701 + 0.15476481481481481481e-2 * t102723 + t99646 + 0.23214722222222222221e-2 * t102725;
    (t102698, t102701, t102706, t102709, t102712, t102715, t102723, t102725, t102727)
}
