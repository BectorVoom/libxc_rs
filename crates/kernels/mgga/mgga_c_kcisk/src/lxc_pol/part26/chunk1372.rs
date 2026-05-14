//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1372/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1372<F: Float>(t35018: F, t9532: F, t2331: F, t32440: F, t6204: F, t6581: F, t113666: F, t115058: F, t115526: F, t115926: F, t118837: F, t118840: F, t118843: F, t118846: F, t118849: F, t118859: F, t2740: F, t32439: F, t33794: F, t33827: F, t33850: F, t9850: F, t9864: F) -> (F, F) {
    let t120091 = t35018 * t9532;
    let t120101 = t6204 * t32440 * t2331 * t6581;
    let t120111 = 0.27777777777777777779e-1 * t9850 * t33850 * t2740 - 0.34722222222222222223e-2 * t120091 - 0.51588271604938271603e-3 * t113666 + t115058 + 0.23214722222222222222e-2 * t118837 - 0.15476481481481481481e-2 * t118840 + 0.69644166666666666666e-2 * t118843 - 0.11607361111111111111e-1 * t118846 - 0.92858888888888888888e-2 * t118849 - 0.40208333333333333334e-2 * t32439 * t120101 - 0.34722222222222222222e-2 * t115926 * t9864 - 0.15476481481481481481e-2 * t118859 - 0.34722222222222222222e-2 * t115526 * t9864 - 0.69444444444444444444e-2 * t33794 * t33827;
    (t120101, t120111)
}
