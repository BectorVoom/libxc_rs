//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1383/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1383<F: Float>(t117120: F, t11986: F, t18325: F, t780: F, t112661: F, t112663: F, t113134: F, t117093: F, t117097: F, t117110: F, t117113: F, t117118: F, t117133: F, t118028: F, t118105: F, t33180: F, t33196: F, t9991: F) -> (F,) {
    let t118439 = 0.10317654320987654321e-2 * t117120;
    let t118443 = t11986 * t780 * t18325;
    let t118449 = -0.10446625e-1 * t117093 + 0.38691203703703703703e-3 * t117097 + 0.40208333333333333334e-2 * t33196 * t118028 - 0.30952962962962962962e-2 * t117110 + 0.25794135802469135802e-2 * t117113 - 0.38691203703703703703e-3 * t117118 - 0.20104166666666666667e-2 * t113134 + t118439 + 0.77382407407407407406e-3 * t112661 - 0.51588271604938271604e-3 * t112663 - 0.77602083333333333334e-3 * t118443 * t118105 + 0.38691203703703703704e-2 * t117133 - 0.10416666666666666667e-1 * t9991 * t33180;
    (t118449,)
}
