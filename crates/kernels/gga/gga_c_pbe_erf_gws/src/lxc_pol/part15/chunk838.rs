//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 838/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk838<F: Float>(t418: F, t950: F, t562: F, t7049: F, t5218: F, t5219: F, t572: F, t610: F, t108: F, t182: F, t267: F, t1764: F) -> (F, F, F, F, F, F) {
    let t7050 = t950 * t418;
    let t7051 = t7050 * t562;
    let t7052 = t7049 * t7051;
    let t7054 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t5218 * t7052;
    let t7055 = t5219 * t572;
    let t7056 = t950 * t610;
    let t7058 = t7055 * t7056 * t562;
    let t7060 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5218 * t7058;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    (t7051, t7054, t7056, t7060, t7062, t7063)
}
