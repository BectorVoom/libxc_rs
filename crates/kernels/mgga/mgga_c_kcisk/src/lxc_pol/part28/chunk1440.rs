//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1440/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1440<F: Float>(t35511: F, t9736: F, t1636: F, t24511: F, t33225: F, t122494: F, t9740: F, t35439: F, t35410: F, t112765: F, t112872: F, t117084: F, t121915: F, t121919: F, t122965: F, t122970: F, t33196: F, t34419: F, t34435: F, t34548: F, t35454: F) -> (F,) {
    let t123160 = t35511 * t9736;
    let t123163 = t33225 * t24511 * t1636;
    let t123180 = t9740 * t122494;
    let t123184 = t35439 * t9736;
    let t123186 = t35410 * t9736;
    let t123189 = 0.92592592592592592593e-2 * t123160 - 0.38801041666666666667e-3 * t34419 * t123163 - 0.20104166666666666667e-2 * t33196 * t123163 + 0.13402777777777777778e-2 * t112872 * t35454 + 0.13402777777777777778e-2 * t112765 * t35454 + 0.13402777777777777778e-2 * t33196 * t122965 - 0.26805555555555555556e-2 * t33196 * t122970 - 0.34722222222222222223e-2 * t9740 * t123163 + 0.34722222222222222223e-2 * t34435 * t34548 + 0.11574074074074074074e-2 * t123180 - 0.41270617283950617283e-2 * t117084 - 0.23214722222222222221e-2 * t121915 - 0.16975308641975308642e-1 * t123184 + 0.92592592592592592593e-2 * t123186 + 0.69644166666666666664e-2 * t121919;
    (t123189,)
}
