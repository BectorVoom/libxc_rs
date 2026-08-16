//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 721/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk721(t14384: f64, t1445: f64, t1457: f64, t2949: f64, t3720: f64, t13619: f64, t13623: f64, t13627: f64, t13634: f64, t14366: f64, t14370: f64, t14374: f64, t14378: f64, t2103: f64, t3040: f64, t317: f64, t3733: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t14385 = t1445 * t14384;
    let t14388 = t1457 * t14384;
    let t14391 = t2949 * t3720;
    let t14392 = t1445 * t14391;
    let t14395 = t13619 - t13623 - 0.23005755572352449806e1_f64 * t813 * t14366 + 0.23005755572352449806e1_f64 * t833 * t14370 + 0.35750489951850426669e0_f64 * t14374 * t317 - 0.35750489951850426669e0_f64 * t797 * t14378 - 0.38342925953920749676e0_f64 * t13627 + t13634 + 0.71500979903700853338e0_f64 * t3733 * t3040 + 0.23005755572352449806e2_f64 * t833 * t14385 + 0.14300195980740170668e1_f64 * t2103 * t14388 - 0.92023022289409799224e1_f64 * t813 * t14392;
    (t14385, t14388, t14391, t14392, t14395)
}
