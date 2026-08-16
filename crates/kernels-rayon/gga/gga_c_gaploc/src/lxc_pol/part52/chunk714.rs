//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 714/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk714(t14302: f64, t1445: f64, t14271: f64, t12054: f64, t12881: f64, t13354: f64, t13356: f64, t13365: f64, t13370: f64, t13374: f64, t13775: f64, t13776: f64, t14299: f64, t1562: f64, t2877: f64, t3702: f64, t574: f64, t597: f64) -> (f64, f64, f64) {
    let t14303 = t1445 * t14302;
    let t14306 = t1445 * t14271;
    let t14313 = t13354 + t13356 + t13365 - t13775 + t13776 - t13370 - t13374 - 0.13803453343411469884e2_f64 * t1562 * t14299 - 0.92023022289409799224e1_f64 * t574 * t14303 + 0.23005755572352449806e2_f64 * t597 * t14306 + 0.71500979903700853338e0_f64 * t3702 * t2877 - 0.21450293971110256002e1_f64 * t12054 * t12881;
    (t14303, t14306, t14313)
}
