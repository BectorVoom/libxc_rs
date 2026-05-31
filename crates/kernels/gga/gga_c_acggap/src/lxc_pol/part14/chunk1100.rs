//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1100/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1100<F: Float>(t1967: F, t9538: F, t1089: F, t15995: F, t2288: F, t598: F, t4643: F, t8484: F, t1980: F, t38798: F, t7458: F, t30375: F, t30397: F, t30398: F, t30406: F, t34349: F, t34352: F, t34362: F, t39209: F, t39213: F, t39217: F, t39222: F, t39226: F, t39228: F, t39230: F) -> F {
    let t39232 = t1967 * t9538;
    let t39236 = t598 * t1089 * t15995 * t2288;
    let t39240 = t598 * t1089 * t4643 * t8484;
    let t39243 = t1980 * t7458 * t38798;
    let t39247 = -F::cast_from(0.75475421495049964964e-2_f64) * t34349 + F::cast_from(0.15724046144802076034e-2_f64) * t39209 + F::cast_from(0.10482697429868050689e-3_f64) * t39213 - F::cast_from(0.10718504529517434243e-3_f64) * t39217 + F::cast_from(0.47172138434406228102e-3_f64) * t39222 + F::cast_from(0.31448092289604152068e-3_f64) * t39226 + F::cast_from(0.18868855373762491241e-2_f64) * t39228 - F::cast_from(0.42874018118069736972e-3_f64) * t39230 - F::cast_from(0.47172138434406228102e-2_f64) * t39232 - F::cast_from(0.42874018118069736972e-3_f64) * t39236 + t34352 - F::cast_from(0.42874018118069736972e-3_f64) * t39240 - F::cast_from(0.28582678745379824648e-3_f64) * t39243 + F::cast_from(0.62896184579208304136e-3_f64) * t30375 - t34362 - t30397 + F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t30398 - t30406;
    t39247
}
