//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 512/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk512(t1645: f64, t2349: f64, t475: f64, t9316: f64, t1445: f64, t188: f64, t9181: f64, t9182: f64, t1457: f64, t9177: f64, t1450: f64, t1456: f64, t1572: f64, t2385: f64, t2386: f64, t3159: f64, t4446: f64, t4507: f64, t4540: f64, t567: f64, t574: f64, t9310: f64, t9313: f64, t9318: f64, t9321: f64, t9324: f64, t9327: f64, t9330: f64) -> (f64, f64, f64) {
    let t9333 = t1645 * t2349;
    let t9338 = t9316 * t475;
    let t9339 = t1445 * t9338;
    let t9342 = t188 * t9181;
    let t9343 = t1645 * t9182;
    let t9346 = t1457 * t9177;
    let t9349 = -0.92023022289409799224e1_f64 * t574 * t9310 - 0.11502877786176224903e2_f64 * t1450 * t9313 + 0.46011511144704899612e1_f64 * t567 * t9318 - 0.71500979903700853338e0_f64 * t4507 * t9321 + 0.14300195980740170668e1_f64 * t1572 * t9324 + 0.35750489951850426669e0_f64 * t1456 * t9327 + 0.10725146985555128001e1_f64 * t9330 * t4446 - 0.21450293971110256002e1_f64 * t2385 * t9333 - 0.25025342966295298669e1_f64 * t3159 * t2386 - 0.92023022289409799224e1_f64 * t574 * t9339 + 0.42900587942220512003e1_f64 * t9342 * t9343 - 0.21450293971110256001e1_f64 * t4540 * t9346;
    (t9333, t9346, t9349)
}
