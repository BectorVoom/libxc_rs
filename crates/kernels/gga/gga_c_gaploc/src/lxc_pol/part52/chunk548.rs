//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 548/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk548<F: Float>(t3560: F, t528: F, t1564: F, t3529: F, t475: F, t1445: F, t3516: F, t4529: F, t11275: F, t549: F, t1000: F, t11386: F, t11389: F, t11392: F, t11395: F, t11402: F, t1429: F, t1562: F, t1646: F, t2859: F, t4446: F, t4527: F, t536: F, t8072: F, t9371: F) -> (F, F, F) {
    let t11405 = t528 * t3560;
    let t11408 = t1564 * t3529;
    let t11409 = t11408 * t475;
    let t11410 = t1445 * t11409;
    let t11413 = t4529 * t3516;
    let t11414 = t11413 * t475;
    let t11415 = t1445 * t11414;
    let t11418 = t549 * t11275;
    let t11421 = -0.31952438294933958063e-1 * t9371 + 0.35750489951850426669e0 * t536 * t11386 + 0.10725146985555128001e1 * t11389 * t4446 - 0.21450293971110256002e1 * t2859 * t11392 - 0.35750489951850426669e0 * t11395 * t1646 + 0.71500979903700853338e0 * t1000 * t8072 + 0.35750489951850426669e0 * t536 * t11402 - 0.35750489951850426669e0 * t11405 * t1646 - 0.69017266717057349418e1 * t1562 * t11410 + 0.27606906686822939767e2 * t4527 * t11415 + 0.39722766613167140743e-1 * t1429 * t11418;
    (t11408, t11413, t11421)
}
