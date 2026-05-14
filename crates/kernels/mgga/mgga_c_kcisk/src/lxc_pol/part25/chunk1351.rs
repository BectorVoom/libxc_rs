//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1351/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1351<F: Float>(t112149: F, t2594: F, t11701: F, t5219: F, t9988: F, t116126: F, t116129: F, t116133: F, t34593: F, t9739: F, t112180: F, t112182: F, t112751: F, t112933: F, t116139: F, t18317: F, t20: F, t2801: F, t2807: F, t33222: F, t33229: F, t34406: F, t34412: F, t34435: F, t34456: F, t654: F, t9720: F) -> (F, F, F, F) {
    let t117597 = t112149 * t2594;
    let t117601 = 6.0 * t11701 * t9988 * t5219;
    let t117613 = 0.15476481481481481481e-2 * t116126;
    let t117616 = 0.10317654320987654321e-2 * t116129;
    let t117618 = 0.30952962962962962962e-2 * t116133;
    let t117621 = t34593 * t9739;
    let t117627 = -0.23280625e-2 * t112933 * t9739 * t34406 - 0.10416666666666666667e-1 * t9720 * t34456 * t2807 - 0.52083333333333333333e-2 * t2801 * t18317 * t654 * t20 * t2807 - t117613 + 0.67013888888888888888e-3 * t112751 - 0.15476481481481481481e-2 * t112180 + t117616 + 0.10317654320987654321e-2 * t112182 - t117618 - 0.92592592592592592594e-2 * t34412 * t33222 + 0.13402777777777777778e-2 * t117621 * t33229 + 0.34722222222222222222e-2 * t34435 * t33229 - 0.61905925925925925924e-2 * t116139;
    (t117597, t117601, t117621, t117627)
}
