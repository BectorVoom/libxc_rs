//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1269/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1269<F: Float>(t12286: F, t6140: F, t2012: F, t303: F, t4110: F, t27342: F, t28397: F, t28403: F, t7895: F, t7901: F, t94621: F, t94624: F, t98706: F, t98709: F, t98712: F, t98715: F, t98719: F) -> (F, F) {
    let t98721 = t12286 * t6140;
    let t98725 = t303 * t4110 * t2012;
    let t98729 = -F::cast_from(0.3684876543209876543e-3_f64) * t94621 - F::cast_from(0.33163888888888888888e-2_f64) * t94624 - F::cast_from(0.44218518518518518517e-2_f64) * t98706 - F::cast_from(0.24872916666666666666e-2_f64) * t98709 + F::cast_from(0.33163888888888888888e-2_f64) * t98712 + F::cast_from(0.16581944444444444444e-2_f64) * t98715 + F::cast_from(0.13901041666666666667e-2_f64) * t7895 * t28403 - F::cast_from(0.55273148148148148147e-3_f64) * t98719 - F::cast_from(0.4946917361111111111e-3_f64) * t98721 * t7901 + F::cast_from(0.16581944444444444444e-2_f64) * t98725 - F::cast_from(0.2782641015625e-3_f64) * t28397 * t27342;
    (t98725, t98729)
}
