//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 635/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk635(t3560: f64, t528: f64, t1564: f64, t3529: f64, t475: f64, t1445: f64, t3516: f64, t4529: f64, t11275: f64, t549: f64, t1000: f64, t11386: f64, t11389: f64, t11392: f64, t11395: f64, t11402: f64, t1429: f64, t1562: f64, t1646: f64, t2859: f64, t4446: f64, t4527: f64, t536: f64, t8072: f64, t9371: f64) -> (f64, f64, f64) {
    let t11405 = t528 * t3560;
    let t11408 = t1564 * t3529;
    let t11409 = t11408 * t475;
    let t11410 = t1445 * t11409;
    let t11413 = t4529 * t3516;
    let t11414 = t11413 * t475;
    let t11415 = t1445 * t11414;
    let t11418 = t549 * t11275;
    let t11421 = -0.31952438294933958063e-1_f64 * t9371 + 0.35750489951850426669e0_f64 * t536 * t11386 + 0.10725146985555128001e1_f64 * t11389 * t4446 - 0.21450293971110256002e1_f64 * t2859 * t11392 - 0.35750489951850426669e0_f64 * t11395 * t1646 + 0.71500979903700853338e0_f64 * t1000 * t8072 + 0.35750489951850426669e0_f64 * t536 * t11402 - 0.35750489951850426669e0_f64 * t11405 * t1646 - 0.69017266717057349418e1_f64 * t1562 * t11410 + 0.27606906686822939767e2_f64 * t4527 * t11415 + 0.39722766613167140743e-1_f64 * t1429 * t11418;
    (t11408, t11413, t11421)
}
