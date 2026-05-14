//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 976/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk976<F: Float>(t30375: F, t30397: F, t30398: F, t30406: F, t34349: F, t34352: F, t34362: F, t39209: F, t39213: F, t39217: F, t39222: F, t39226: F, t39228: F, t39230: F, t39232: F, t39236: F, t39240: F, t39243: F) -> (F,) {
    let t39247 = -0.75475421495049964964e-2 * t34349 + 0.15724046144802076034e-2 * t39209 + 0.10482697429868050689e-3 * t39213 - 0.10718504529517434243e-3 * t39217 + 0.47172138434406228102e-3 * t39222 + 0.31448092289604152068e-3 * t39226 + 0.18868855373762491241e-2 * t39228 - 0.42874018118069736972e-3 * t39230 - 0.47172138434406228102e-2 * t39232 - 0.42874018118069736972e-3 * t39236 + t34352 - 0.42874018118069736972e-3 * t39240 - 0.28582678745379824648e-3 * t39243 + 0.62896184579208304136e-3 * t30375 - t34362 - t30397 + 35.0 / 432.0 * t30398 - t30406;
    (t39247,)
}
