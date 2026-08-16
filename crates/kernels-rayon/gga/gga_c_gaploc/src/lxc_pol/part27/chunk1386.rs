//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1386/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1386(t11981: f64, t1305: f64, t1445: f64, t1450: f64, t1603: f64, t30374: f64, t30378: f64, t30379: f64, t30381: f64, t30387: f64, t34358: f64, t34361: f64, t34366: f64, t34370: f64, t34374: f64, t34377: f64, t34381: f64, t34383: f64, t34386: f64, t3701: f64, t4667: f64) -> f64 {
    let t38535 = t34358 - 0.23005755572352449806e1_f64 * t1450 * t1445 * t11981 * t1305 + t34361 + t34366 - t34370 + t34374 + t34377 - t34381 + t30374 - t30378 + t34383 - 0.38342925953920749677e0_f64 * t30379 - 0.76685851907841499354e0_f64 * t30381 + 0.10224780254378866581e1_f64 * t30387 - t34386 + 0.71500979903700853338e0_f64 * t1603 * t3701 * t4667;
    t38535
}
