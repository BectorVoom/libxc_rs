//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 473/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk473<F: Float>(t360: F, t4007: F, t1265: F, t370: F, t4060: F, t373: F, t117: F, t441: F, t381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4082 = t360 * t360;
    let t4083 = F::cast_from(1.0_f64) / t4082;
    let t4087 = F::cast_from(0.12361111111111111111e-1_f64) * t4007;
    let t4099 = t1265 * t370;
    let t4100 = F::cast_from(1.0_f64) / t4099;
    let t4108 = F::cast_from(0.40256666666666666667e0_f64) * t4007;
    let t4115 = F::cast_from(0.27595e0_f64) * t4060;
    let t4125 = t1265 * t1265;
    let t4126 = F::cast_from(1.0_f64) / t4125;
    let t4128 = t373 * t373;
    let t4129 = F::cast_from(1.0_f64) / t4128;
    let t4141 = t117 * t441;
    let t4143 = F::cast_from(1.0_f64) / t381 / t4141;
    (t4082, t4083, t4087, t4100, t4108, t4115, t4125, t4126, t4128, t4129, t4143)
}
