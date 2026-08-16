//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1010/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1010<F: Float>(t30217: F, t2297: F, t4210: F, t13364: F, t31115: F, t1: F, t1170: F, t2065: F, t8461: F, t3196: F, t30157: F, t30162: F, t30171: F, t30181: F, t30184: F, t30185: F, t30187: F, t30192: F, t30195: F, t30198: F, t30200: F, t30201: F, t30203: F, t30212: F, t30220: F) -> (F, F, F, F) {
    let t33936 = F::cast_from(0.27953859812981468505e-2_f64) * t30217;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33941 = F::cast_from(0.10718504529517434243e-2_f64) * t33940;
    let t33944 = t1170 * t2065 * t8461 * t1;
    let t33945 = t2297 * t3196;
    let t33947 = t33944 * t13364 * t33945;
    let t33949 = -F::cast_from(0.31448092289604152068e-2_f64) * t30157 + F::cast_from(0.12579236915841660827e-2_f64) * t30162 - t30171 - t30181 + t30184 + F::cast_from(0.18868855373762491241e-2_f64) * t30185 - F::cast_from(0.40015750243531754508e-2_f64) * t30187 + t30192 - t30195 - t30198 - t30200 - F::cast_from(0.47172138434406228102e-2_f64) * t30201 + F::cast_from(0.32012600194825403606e-1_f64) * t30203 - F::cast_from(0.12579236915841660827e-2_f64) * t30212 - t33936 + F::cast_from(0.21437009059034868486e-2_f64) * t30220 - t33941 - F::cast_from(0.64311027177104605458e-2_f64) * t33947;
    (t33938, t33944, t33945, t33949)
}
