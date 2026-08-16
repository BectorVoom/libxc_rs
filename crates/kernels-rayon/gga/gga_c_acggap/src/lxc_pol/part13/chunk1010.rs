//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1010/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1010(t30217: f64, t2297: f64, t4210: f64, t13364: f64, t31115: f64, t1: f64, t1170: f64, t2065: f64, t8461: f64, t3196: f64, t30157: f64, t30162: f64, t30171: f64, t30181: f64, t30184: f64, t30185: f64, t30187: f64, t30192: f64, t30195: f64, t30198: f64, t30200: f64, t30201: f64, t30203: f64, t30212: f64, t30220: f64) -> (f64, f64, f64, f64) {
    let t33936 = 0.27953859812981468505e-2_f64 * t30217;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33941 = 0.10718504529517434243e-2_f64 * t33940;
    let t33944 = t1170 * t2065 * t8461 * t1;
    let t33945 = t2297 * t3196;
    let t33947 = t33944 * t13364 * t33945;
    let t33949 = -0.31448092289604152068e-2_f64 * t30157 + 0.12579236915841660827e-2_f64 * t30162 - t30171 - t30181 + t30184 + 0.18868855373762491241e-2_f64 * t30185 - 0.40015750243531754508e-2_f64 * t30187 + t30192 - t30195 - t30198 - t30200 - 0.47172138434406228102e-2_f64 * t30201 + 0.32012600194825403606e-1_f64 * t30203 - 0.12579236915841660827e-2_f64 * t30212 - t33936 + 0.21437009059034868486e-2_f64 * t30220 - t33941 - 0.64311027177104605458e-2_f64 * t33947;
    (t33938, t33944, t33945, t33949)
}
