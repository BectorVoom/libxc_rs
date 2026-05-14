//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1129/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1129<F: Float>(t27342: F, t28397: F, t28403: F, t7895: F, t7901: F, t94621: F, t94624: F, t98706: F, t98709: F, t98712: F, t98715: F, t98719: F, t98721: F, t98725: F, t1598: F, t51799: F) -> (F, F) {
    let t98729 = -0.3684876543209876543e-3 * t94621 - 0.33163888888888888888e-2 * t94624 - 0.44218518518518518517e-2 * t98706 - 0.24872916666666666666e-2 * t98709 + 0.33163888888888888888e-2 * t98712 + 0.16581944444444444444e-2 * t98715 + 0.13901041666666666667e-2 * t7895 * t28403 - 0.55273148148148148147e-3 * t98719 - 0.4946917361111111111e-3 * t98721 * t7901 + 0.16581944444444444444e-2 * t98725 - 0.2782641015625e-3 * t28397 * t27342;
    let t98733 = t51799 * t1598;
    (t98729, t98733)
}
