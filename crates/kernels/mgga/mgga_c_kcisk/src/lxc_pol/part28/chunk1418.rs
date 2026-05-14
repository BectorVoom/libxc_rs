//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1418/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1418<F: Float>(t2647: F, t34403: F, t7261: F, t7644: F, t2804: F, t35468: F, t4419: F, t10005: F, t34465: F, t112856: F, t117773: F, t117808: F, t117810: F, t117812: F, t117814: F, t121171: F, t121174: F, t121181: F, t121214: F, t34412: F, t34419: F, t34501: F) -> (F, F) {
    let t122539 = t7261 * t34403 * t2647 * t7644;
    let t122543 = t2804 * t4419 * t35468;
    let t122545 = t10005 * t34465;
    let t122551 = -0.92592592592592592593e-2 * t117773 + 0.15476481481481481481e-2 * t121171 - 0.23214722222222222221e-2 * t121174 + 0.61905925925925925924e-2 * t121181 + t117808 + t117810 + t117812 + t117814 - 0.23280625e-2 * t34419 * t122539 + 0.17361111111111111111e-2 * t122543 - 0.92592592592592592593e-2 * t122545 + 0.11574074074074074074e-2 * t112856 + 0.23214722222222222221e-2 * t121214 - 0.18518518518518518519e-1 * t34412 * t34501;
    (t122539, t122551)
}
